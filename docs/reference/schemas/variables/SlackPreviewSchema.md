[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / SlackPreviewSchema

# Variable: SlackPreviewSchema

> `const` **SlackPreviewSchema**: `ZodObject`\<\{ `detected_format`: `ZodString`; `line_count`: `ZodNumber`; `message_count`: `ZodNumber`; `warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:410](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L410)
