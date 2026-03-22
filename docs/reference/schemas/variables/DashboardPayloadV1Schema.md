[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / DashboardPayloadV1Schema

# Variable: DashboardPayloadV1Schema

> `const` **DashboardPayloadV1Schema**: `ZodObject`\<\{ `incident_count`: `ZodNumber`; `incidents`: `ZodArray`\<`ZodObject`\<\{ `external_id`: `ZodNullable`\<`ZodString`\>; `id`: `ZodNumber`; `mtta_seconds`: `ZodNullable`\<`ZodNumber`\>; `mttr_seconds`: `ZodNullable`\<`ZodNumber`\>; `severity`: `ZodNullable`\<`ZodString`\>; `title`: `ZodString`; `warning_count`: `ZodNumber`; \}, `$strip`\>\>; `severity_counts`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `severity`: `ZodString`; \}, `$strip`\>\>; `version`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:108](https://github.com/saagpatel/IncidentReview/blob/0e2d0e4b49c7d6af9d6183f418bd05b1e0fe7fe9/src/lib/schemas.ts#L108)
