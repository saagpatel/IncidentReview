pub mod evidence;
pub mod embeddings;
pub mod draft;
pub mod guardrails;
pub mod llm;
pub mod ollama;
pub mod retrieve;

#[cfg(test)]
mod tests {
    use super::ollama::OllamaClient;
    use super::{evidence::EvidenceStore, guardrails::enforce_citations};
    use tempfile::tempdir;

    #[test]
    fn enforces_localhost_only_base_url() {
        assert!(OllamaClient::new("http://127.0.0.1:11434").is_ok());
        assert!(OllamaClient::new("http://127.0.0.1").is_ok());

        assert!(OllamaClient::new("http://localhost:11434").is_err());
        assert!(OllamaClient::new("http://0.0.0.0:11434").is_err());
        assert!(OllamaClient::new("http://[::1]:11434").is_err());
        assert!(OllamaClient::new("https://example.com").is_err());
        assert!(OllamaClient::new("HTTP://127.0.0.1:11434").is_err());

        // Harden against prefix-based bypasses.
        assert!(OllamaClient::new("http://127.0.0.1.evil.com:11434").is_err());
        assert!(OllamaClient::new("http://127.0.0.1@evil.com:11434").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:0").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:99999").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:11434/").is_ok()); // trailing slash is trimmed
        assert!(OllamaClient::new("http://127.0.0.1:11434/api").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:11434?x=1").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:11434#frag").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:11434@evil.com").is_err());
        assert!(OllamaClient::new("http://127.0.0.1:11434:123").is_err());
        assert!(OllamaClient::new("http://127.0.0.1%0a:11434").is_err());
    }

    #[test]
    fn evidence_store_roundtrip() {
        let dir = tempdir().expect("tempdir");
        let store = EvidenceStore::open(dir.path().to_path_buf());
        let source = store
            .add_source(super::evidence::EvidenceAddSourceInput {
                source_type: super::evidence::EvidenceSourceType::FreeformText,
                origin: super::evidence::EvidenceOrigin {
                    kind: "paste".to_string(),
                    path: None,
                },
                label: "test".to_string(),
                created_at: "2026-02-10T00:00:00Z".to_string(),
                text: Some("hello world".to_string()),
            })
            .expect("add_source");
        store
            .build_chunks(Some(source.source_id.clone()), "2026-02-10T00:00:00Z")
            .expect("build_chunks");
        let chunks = store
            .list_chunks(super::evidence::EvidenceQueryStore {
                include_text: false,
                source_id: Some(source.source_id.clone()),
            })
            .expect("list");
        assert_eq!(chunks.len(), 1);
        let chunk = store.get_chunk(&chunks[0].chunk_id).expect("get");
        assert_eq!(chunk.source_id, source.source_id);
        assert!(chunk.text.contains("hello world"));
    }

    #[test]
    fn evidence_store_rejects_empty_path_for_directory_sources() {
        let dir = tempdir().expect("tempdir");
        let store = EvidenceStore::open(dir.path().to_path_buf());
        let err = store
            .add_source(super::evidence::EvidenceAddSourceInput {
                source_type: super::evidence::EvidenceSourceType::SanitizedExport,
                origin: super::evidence::EvidenceOrigin {
                    kind: "directory".to_string(),
                    path: Some("   ".to_string()),
                },
                label: "bad".to_string(),
                created_at: "2026-03-14T00:00:00Z".to_string(),
                text: None,
            })
            .expect_err("empty path should fail");

        assert_eq!(err.code, "AI_EVIDENCE_SOURCE_INVALID");
        assert!(err.message.contains("path is required"));
    }

    #[test]
    fn evidence_store_rejects_source_type_origin_mismatches() {
        let dir = tempdir().expect("tempdir");
        let store = EvidenceStore::open(dir.path().to_path_buf());
        let err = store
            .add_source(super::evidence::EvidenceAddSourceInput {
                source_type: super::evidence::EvidenceSourceType::FreeformText,
                origin: super::evidence::EvidenceOrigin {
                    kind: "file".to_string(),
                    path: Some("/tmp/evidence.txt".to_string()),
                },
                label: "bad".to_string(),
                created_at: "2026-03-14T00:00:00Z".to_string(),
                text: None,
            })
            .expect_err("freeform text should require paste");

        assert_eq!(err.code, "AI_EVIDENCE_SOURCE_INVALID");
        assert!(err.message.contains("paste origin"));
    }

    #[test]
    fn evidence_store_assigns_distinct_ids_for_distinct_paste_content() {
        let dir = tempdir().expect("tempdir");
        let store = EvidenceStore::open(dir.path().to_path_buf());
        let first = store
            .add_source(super::evidence::EvidenceAddSourceInput {
                source_type: super::evidence::EvidenceSourceType::FreeformText,
                origin: super::evidence::EvidenceOrigin {
                    kind: "paste".to_string(),
                    path: None,
                },
                label: "first".to_string(),
                created_at: "2026-03-14T00:00:00Z".to_string(),
                text: Some("vendor saturation".to_string()),
            })
            .expect("first");
        let second = store
            .add_source(super::evidence::EvidenceAddSourceInput {
                source_type: super::evidence::EvidenceSourceType::FreeformText,
                origin: super::evidence::EvidenceOrigin {
                    kind: "paste".to_string(),
                    path: None,
                },
                label: "second".to_string(),
                created_at: "2026-03-14T00:00:00Z".to_string(),
                text: Some("customer messaging".to_string()),
            })
            .expect("second");

        assert_ne!(first.source_id, second.source_id);
    }

    #[test]
    fn citation_guard_rejects_missing_citations() {
        assert!(enforce_citations("no citations here").is_err());
        assert!(enforce_citations("supported [[chunk:abc123]] citation").is_ok());
    }
}
