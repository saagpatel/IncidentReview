[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / SanitizedExportManifestSchema

# Variable: SanitizedExportManifestSchema

> `const` **SanitizedExportManifestSchema**: `ZodObject`\<\{ `app_version`: `ZodString`; `export_time`: `ZodString`; `files`: `ZodArray`\<`ZodObject`\<\{ `bytes`: `ZodNumber`; `filename`: `ZodString`; `sha256`: `ZodString`; \}, `$strip`\>\>; `incident_count`: `ZodNumber`; `manifest_version`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:385](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L385)
