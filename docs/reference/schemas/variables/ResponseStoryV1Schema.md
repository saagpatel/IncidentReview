[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / ResponseStoryV1Schema

# Variable: ResponseStoryV1Schema

> `const` **ResponseStoryV1Schema**: `ZodObject`\<\{ `time_to_mitigation_buckets`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `key`: `ZodString`; `label`: `ZodString`; \}, `$strip`\>\>; `time_to_resolve_buckets`: `ZodArray`\<`ZodObject`\<\{ `count`: `ZodNumber`; `incident_ids`: `ZodArray`\<`ZodNumber`\>; `key`: `ZodString`; `label`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:150](https://github.com/saagpatel/IncidentReview/blob/0e2d0e4b49c7d6af9d6183f418bd05b1e0fe7fe9/src/lib/schemas.ts#L150)
