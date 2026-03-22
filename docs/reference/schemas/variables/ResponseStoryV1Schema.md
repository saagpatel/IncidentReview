[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / ResponseStoryV1Schema

# Variable: ResponseStoryV1Schema

> `const` **ResponseStoryV1Schema**: `ZodObject`\<\{ `time_to_mitigation_buckets`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `key`: `ZodString`; `label`: `ZodString`; \}, `$strip`\>\>; `time_to_resolve_buckets`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `key`: `ZodString`; `label`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:150](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/schemas.ts#L150)
