[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [schemas](../README.md) / SlackPreviewSchema

# Variable: SlackPreviewSchema

> `const` **SlackPreviewSchema**: `ZodObject`\<\{ `detected_format`: `ZodString`; `line_count`: `ZodNumber`; `message_count`: `ZodNumber`; `warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:410](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/schemas.ts#L410)
