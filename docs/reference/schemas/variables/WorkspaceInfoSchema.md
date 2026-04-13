[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / WorkspaceInfoSchema

# Variable: WorkspaceInfoSchema

> `const` **WorkspaceInfoSchema**: `ZodObject`\<\{ `current_db_path`: `ZodString`; `load_error`: `ZodOptional`\<`ZodNullable`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; `retryable`: `ZodBoolean`; \}, `$strip`\>\>\>; `recent_db_paths`: `ZodArray`\<`ZodString`\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:404](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L404)
