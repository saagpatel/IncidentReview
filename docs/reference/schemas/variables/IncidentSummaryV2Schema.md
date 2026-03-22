[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / IncidentSummaryV2Schema

# Variable: IncidentSummaryV2Schema

> `const` **IncidentSummaryV2Schema**: `ZodObject`\<\{ `detection_source`: `ZodNullable`\<`ZodString`\>; `external_id`: `ZodNullable`\<`ZodString`\>; `id`: `ZodNumber`; `it_awareness_lag_seconds`: `ZodNullable`\<`ZodNumber`\>; `mttr_seconds`: `ZodNullable`\<`ZodNumber`\>; `service`: `ZodNullable`\<`ZodString`\>; `severity`: `ZodNullable`\<`ZodString`\>; `time_to_mitigation_seconds`: `ZodNullable`\<`ZodNumber`\>; `title`: `ZodString`; `vendor`: `ZodNullable`\<`ZodString`\>; `warning_count`: `ZodNumber`; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:155](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L155)
