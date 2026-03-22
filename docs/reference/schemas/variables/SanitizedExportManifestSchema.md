[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / SanitizedExportManifestSchema

# Variable: SanitizedExportManifestSchema

> `const` **SanitizedExportManifestSchema**: `ZodObject`\<\{ `app_version`: `ZodString`; `export_time`: `ZodString`; `files`: `ZodArray`\<`ZodObject`\<\{ `bytes`: `ZodNumber`; `filename`: `ZodString`; `sha256`: `ZodString`; \}, `$strip`\>\>; `incident_count`: `ZodNumber`; `manifest_version`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:385](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L385)
