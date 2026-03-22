[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / WorkspaceInfoSchema

# Variable: WorkspaceInfoSchema

> `const` **WorkspaceInfoSchema**: `ZodObject`\<\{ `current_db_path`: `ZodString`; `load_error`: `ZodOptional`\<`ZodNullable`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; `retryable`: `ZodBoolean`; \}, `$strip`\>\>\>; `recent_db_paths`: `ZodArray`\<`ZodString`\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:404](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L404)
