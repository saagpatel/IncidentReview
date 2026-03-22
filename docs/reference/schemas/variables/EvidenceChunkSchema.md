[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / EvidenceChunkSchema

# Variable: EvidenceChunkSchema

> `const` **EvidenceChunkSchema**: `ZodObject`\<\{ `chunk_id`: `ZodString`; `meta`: `ZodObject`\<\{ `incident_keys`: `ZodOptional`\<`ZodNullable`\<`ZodArray`\<`ZodString`\>\>\>; `kind`: `ZodString`; `time_range`: `ZodOptional`\<`ZodNullable`\<`ZodObject`\<\{ `end_ts`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `start_ts`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; \}, `$strip`\>\>\>; \}, `$strip`\>; `ordinal`: `ZodNumber`; `source_id`: `ZodString`; `text`: `ZodString`; `text_sha256`: `ZodString`; `token_count_est`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:252](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/schemas.ts#L252)
