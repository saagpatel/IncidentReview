[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [schemas](../README.md) / EvidenceQueryResponseSchema

# Variable: EvidenceQueryResponseSchema

> `const` **EvidenceQueryResponseSchema**: `ZodObject`\<\{ `hits`: `ZodArray`\<`ZodObject`\<\{ `chunk_id`: `ZodString`; `citation`: `ZodObject`\<\{ `chunk_id`: `ZodString`; `locator`: `ZodObject`\<\{ `char_range`: `ZodOptional`\<`ZodNullable`\<...\>\>; `ordinal`: `ZodNumber`; `source_id`: `ZodString`; `text_sha256`: `ZodString`; \}, `$strip`\>; \}, `$strip`\>; `score`: `ZodNumber`; `snippet`: `ZodString`; `source_id`: `ZodString`; \}, `$strip`\>\>; \}, `$strip`\>

Defined in: [src/lib/schemas.ts:287](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/schemas.ts#L287)
