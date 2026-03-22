[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / JiraImportSummarySchema

# Variable: JiraImportSummarySchema

> `const` **JiraImportSummarySchema**: `ZodObject`\<\{ `conflicts`: `ZodArray`\<`ZodObject`\<\{ `external_id`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `fingerprint`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `reason`: `ZodString`; `row`: `ZodNumber`; \}, `$strip`\>\>; `inserted`: `ZodNumber`; `skipped`: `ZodNumber`; `updated`: `ZodNumber`; `warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:60](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/schemas.ts#L60)
