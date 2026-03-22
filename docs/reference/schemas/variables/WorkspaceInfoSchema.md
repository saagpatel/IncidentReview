[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / WorkspaceInfoSchema

# Variable: WorkspaceInfoSchema

> `const` **WorkspaceInfoSchema**: `ZodObject`\<\{ `current_db_path`: `ZodString`; `load_error`: `ZodOptional`\<`ZodNullable`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; `retryable`: `ZodBoolean`; \}, `$strip`\>\>\>; `recent_db_paths`: `ZodArray`\<`ZodString`\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:404](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/schemas.ts#L404)
