[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / EvidenceChunkSummaryListSchema

# Variable: EvidenceChunkSummaryListSchema

> `const` **EvidenceChunkSummaryListSchema**: `ZodArray`\<`ZodObject`\<\{ `chunk_id`: `ZodString`; `meta`: `ZodObject`\<\{ `incident_keys`: `ZodOptional`\<`ZodNullable`\<`ZodArray`\<`ZodString`\>\>\>; `kind`: `ZodString`; `time_range`: `ZodOptional`\<`ZodNullable`\<`ZodObject`\<\{ `end_ts`: `ZodOptional`\<`ZodNullable`\<...\>\>; `start_ts`: `ZodOptional`\<`ZodNullable`\<...\>\>; \}, `$strip`\>\>\>; \}, `$strip`\>; `ordinal`: `ZodNumber`; `source_id`: `ZodString`; `text_sha256`: `ZodString`; `token_count_est`: `ZodNumber`; \}, `$strip`\>\>

Defined in: [src/lib/schemas.ts:250](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/schemas.ts#L250)
