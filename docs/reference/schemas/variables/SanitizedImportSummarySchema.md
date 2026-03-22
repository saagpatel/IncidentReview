[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / SanitizedImportSummarySchema

# Variable: SanitizedImportSummarySchema

> `const` **SanitizedImportSummarySchema**: `ZodObject`\<\{ `import_warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; `inserted_incidents`: `ZodNumber`; `inserted_timeline_events`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:393](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L393)
