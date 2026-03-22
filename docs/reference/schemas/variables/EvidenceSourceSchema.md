[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / EvidenceSourceSchema

# Variable: EvidenceSourceSchema

> `const` **EvidenceSourceSchema**: `ZodObject`\<\{ `created_at`: `ZodString`; `label`: `ZodString`; `origin`: `ZodObject`\<\{ `kind`: `ZodString`; `path`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; \}, `$strip`\>; `source_id`: `ZodString`; `type`: `ZodEnum`\<\{ `freeform_text`: `"freeform_text"`; `incident_report_md`: `"incident_report_md"`; `sanitized_export`: `"sanitized_export"`; `slack_transcript`: `"slack_transcript"`; \}\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:220](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L220)
