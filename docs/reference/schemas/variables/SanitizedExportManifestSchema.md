[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / SanitizedExportManifestSchema

# Variable: SanitizedExportManifestSchema

> `const` **SanitizedExportManifestSchema**: `ZodObject`\<\{ `app_version`: `ZodString`; `export_time`: `ZodString`; `files`: `ZodArray`\<`ZodObject`\<\{ `bytes`: `ZodNumber`; `filename`: `ZodString`; `sha256`: `ZodString`; \}, `$strip`\>\>; `incident_count`: `ZodNumber`; `manifest_version`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:385](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/schemas.ts#L385)
