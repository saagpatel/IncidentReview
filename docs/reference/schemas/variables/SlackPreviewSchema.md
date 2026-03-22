[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / SlackPreviewSchema

# Variable: SlackPreviewSchema

> `const` **SlackPreviewSchema**: `ZodObject`\<\{ `detected_format`: `ZodString`; `line_count`: `ZodNumber`; `message_count`: `ZodNumber`; `warnings`: `ZodArray`\<`ZodObject`\<\{ `code`: `ZodString`; `details`: `ZodOptional`\<`ZodNullable`\<`ZodString`\>\>; `message`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:410](https://github.com/saagpatel/IncidentReview/blob/0e2d0e4b49c7d6af9d6183f418bd05b1e0fe7fe9/src/lib/schemas.ts#L410)
