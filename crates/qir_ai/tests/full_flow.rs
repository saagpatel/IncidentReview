use qir_ai::draft::{draft_section_with_llm, AiDraftSectionRequest, SectionId};
use qir_ai::embeddings::Embedder;
use qir_ai::evidence::{
    AiIndexBuildInput, EvidenceAddSourceInput, EvidenceOrigin, EvidenceSourceType, EvidenceStore, IndexStore,
};
use qir_ai::llm::Llm;
use qir_ai::retrieve::query_with_embedder;
use qir_core::ai_drafts::{create_ai_draft, list_ai_drafts, AiDraftSectionType, CreateAiDraftInput};
use qir_core::db;
use qir_core::error::AppError;
use tempfile::tempdir;

struct VendorEmbedder;

impl Embedder for VendorEmbedder {
    fn embed(&self, _model: &str, input: &str) -> Result<Vec<f32>, AppError> {
        let lower = input.to_ascii_lowercase();
        let vendor = lower.matches("vendor").count() as f32;
        let saturation = lower.matches("saturation").count() as f32;
        let customer = lower.matches("customer").count() as f32;
        Ok(vec![vendor, saturation, customer])
    }
}

struct MockLlm {
    out: String,
}

impl Llm for MockLlm {
    fn generate(&self, _model: &str, _prompt: &str) -> Result<String, AppError> {
        Ok(self.out.clone())
    }
}

#[test]
fn selected_source_query_can_be_drafted_and_persisted() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path().join("full-flow");
    let evidence = EvidenceStore::open(root.clone());

    let source = evidence
        .add_source(EvidenceAddSourceInput {
            source_type: EvidenceSourceType::FreeformText,
            origin: EvidenceOrigin {
                kind: "paste".to_string(),
                path: None,
            },
            label: "Desktop AI source".to_string(),
            created_at: "2026-03-14T00:00:00Z".to_string(),
            text: Some(
                "Incident INC-1 failed over after vendor saturation. Mitigation completed in 15 minutes. Customers saw intermittent errors. Monitoring alerted before customer reports.".to_string(),
            ),
        })
        .expect("add_source");

    evidence
        .build_chunks(Some(source.source_id.clone()), "2026-03-14T00:00:00Z")
        .expect("build_chunks");

    let index = IndexStore::open(root);
    index
        .build_with_embedder(
            &evidence,
            &VendorEmbedder,
            AiIndexBuildInput {
                model: "mock-embed".to_string(),
                source_id: Some(source.source_id.clone()),
                updated_at: "2026-03-14T00:00:00Z".to_string(),
            },
        )
        .expect("build_index");

    let query = query_with_embedder(
        &evidence,
        &index,
        &VendorEmbedder,
        "vendor saturation",
        5,
        Some(std::slice::from_ref(&source.source_id)),
    )
    .expect("query");

    assert_eq!(query.hits.len(), 1);
    assert_eq!(query.hits[0].source_id, source.source_id);
    assert!(query.hits[0].snippet.contains("vendor saturation"));

    let chunk_id = query.hits[0].chunk_id.clone();
    let llm = MockLlm {
        out: format!(
            "Vendor saturation caused a customer-visible failover, but mitigation completed quickly [[chunk:{chunk_id}]]"
        ),
    };
    let draft = draft_section_with_llm(
        &evidence,
        &llm,
        "mock-llm",
        AiDraftSectionRequest {
            section_id: SectionId::ExecSummary,
            quarter_label: "Q1 2026".to_string(),
            prompt: "Draft an executive summary based on the evidence.".to_string(),
            citation_chunk_ids: vec![chunk_id.clone()],
        },
    )
    .expect("draft");

    assert!(draft.markdown.contains("[[chunk:"));
    assert_eq!(draft.citations.len(), 1);
    assert_eq!(draft.citations[0].chunk_id, chunk_id);

    let mut conn = db::open_in_memory().expect("open");
    db::migrate(&mut conn).expect("migrate");

    let stored = create_ai_draft(
        &conn,
        CreateAiDraftInput {
            quarter_label: "Q1 2026".to_string(),
            section_type: AiDraftSectionType::ExecSummary,
            draft_text: draft.markdown.clone(),
            citation_chunk_ids: vec![query.hits[0].chunk_id.clone()],
            model_name: draft.model_name.clone(),
            model_params_hash: draft.model_params_hash.clone(),
            prompt_template_version: draft.prompt_template_version.clone(),
            created_at: "2026-03-14T00:00:00Z".to_string(),
            parent_draft_id: None,
            revision_notes: None,
            branch_label: None,
        },
    )
    .expect("store_draft");

    let listed = list_ai_drafts(&conn, Some("Q1 2026")).expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, stored.id);
    assert_eq!(listed[0].citation_chunk_ids, vec![chunk_id]);
}

#[test]
fn selected_source_query_respects_source_filter() {
    let dir = tempdir().expect("tempdir");
    let root = dir.path().join("source-filter");
    let evidence = EvidenceStore::open(root.clone());

    let source_a = evidence
        .add_source(EvidenceAddSourceInput {
            source_type: EvidenceSourceType::FreeformText,
            origin: EvidenceOrigin {
                kind: "paste".to_string(),
                path: None,
            },
            label: "A".to_string(),
            created_at: "2026-03-14T00:00:00Z".to_string(),
            text: Some("Vendor saturation triggered a failover.".to_string()),
        })
        .expect("add_source_a");
    let source_b = evidence
        .add_source(EvidenceAddSourceInput {
            source_type: EvidenceSourceType::FreeformText,
            origin: EvidenceOrigin {
                kind: "paste".to_string(),
                path: None,
            },
            label: "B".to_string(),
            created_at: "2026-03-14T00:00:00Z".to_string(),
            text: Some("Customer messaging improved after the incident.".to_string()),
        })
        .expect("add_source_b");

    evidence
        .build_chunks(None, "2026-03-14T00:00:00Z")
        .expect("build_chunks");

    let index = IndexStore::open(root);
    index
        .build_with_embedder(
            &evidence,
            &VendorEmbedder,
            AiIndexBuildInput {
                model: "mock-embed".to_string(),
                source_id: None,
                updated_at: "2026-03-14T00:00:00Z".to_string(),
            },
        )
        .expect("build_index");

    let filtered = query_with_embedder(
        &evidence,
        &index,
        &VendorEmbedder,
        "vendor saturation",
        5,
        Some(std::slice::from_ref(&source_b.source_id)),
    )
    .expect("filtered_query");

    assert_eq!(filtered.hits.len(), 1);
    assert_eq!(filtered.hits[0].source_id, source_b.source_id);
    assert!(!filtered.hits[0].snippet.contains("vendor saturation"));

    let selected = query_with_embedder(
        &evidence,
        &index,
        &VendorEmbedder,
        "vendor saturation",
        5,
        Some(std::slice::from_ref(&source_a.source_id)),
    )
    .expect("selected_query");

    assert_eq!(selected.hits.len(), 1);
    assert_eq!(selected.hits[0].source_id, source_a.source_id);
    assert_ne!(selected.hits[0].source_id, source_b.source_id);
}
