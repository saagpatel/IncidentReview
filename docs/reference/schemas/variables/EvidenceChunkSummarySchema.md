[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / EvidenceChunkSummarySchema

# Variable: EvidenceChunkSummarySchema

> `const` **EvidenceChunkSummarySchema**: `ZodObject`\<\{ `chunk_id`: `ZodString`; `meta`: `ZodObject`\<\{ `incident_keys`: `ZodOptional`\<`ZodNullable`\<`ZodArray`\<`ZodString`\>\>\>; `kind`: `ZodString`; `time_range`: `ZodOptional`\<`ZodNullable`\<`ZodObject`\<\{ `end_ts`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `start_ts`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; \}, `$strip`\>\>\>; \}, `$strip`\>; `ordinal`: `ZodNumber`; `source_id`: `ZodString`; `text_sha256`: `ZodString`; `token_count_est`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:241](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L241)
