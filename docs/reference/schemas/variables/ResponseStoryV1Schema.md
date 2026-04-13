[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / ResponseStoryV1Schema

# Variable: ResponseStoryV1Schema

> `const` **ResponseStoryV1Schema**: `ZodObject`\<\{ `time_to_mitigation_buckets`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `key`: `ZodString`; `label`: `ZodString`; \}, `$strip`\>\>; `time_to_resolve_buckets`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `key`: `ZodString`; `label`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:150](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L150)
