[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / SanitizedImportSummarySchema

# Variable: SanitizedImportSummarySchema

> `const` **SanitizedImportSummarySchema**: `ZodObject`\<\{ `import_warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; `inserted_incidents`: `ZodNumber`; `inserted_timeline_events`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:393](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L393)
