[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / JiraImportSummarySchema

# Variable: JiraImportSummarySchema

> `const` **JiraImportSummarySchema**: `ZodObject`\<\{ `conflicts`: `ZodArray`\<`ZodObject`\<\{ `external_id`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `fingerprint`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `reason`: `ZodString`; `row`: `ZodNumber`; \}, `$strip`\>\>; `inserted`: `ZodNumber`; `skipped`: `ZodNumber`; `updated`: `ZodNumber`; `warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:60](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L60)
