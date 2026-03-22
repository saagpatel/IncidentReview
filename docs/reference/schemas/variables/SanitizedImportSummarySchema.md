[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / SanitizedImportSummarySchema

# Variable: SanitizedImportSummarySchema

> `const` **SanitizedImportSummarySchema**: `ZodObject`\<\{ `import_warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; `inserted_incidents`: `ZodNumber`; `inserted_timeline_events`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:393](https://github.com/saagpatel/IncidentReview/blob/0e2d0e4b49c7d6af9d6183f418bd05b1e0fe7fe9/src/lib/schemas.ts#L393)
