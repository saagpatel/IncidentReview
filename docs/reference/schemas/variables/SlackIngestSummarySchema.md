[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / SlackIngestSummarySchema

# Variable: SlackIngestSummarySchema

> `const` **SlackIngestSummarySchema**: `ZodObject`\<\{ `detected_format`: `ZodString`; `incident_created`: `ZodBoolean`; `incident_id`: `ZodNumber`; `inserted_events`: `ZodNumber`; `warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:417](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L417)
