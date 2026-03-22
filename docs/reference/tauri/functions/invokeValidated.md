[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [tauri](../README.md) / invokeValidated

# Function: invokeValidated()

> **invokeValidated**\<`T`\>(`command`, `args`, `schema`): `Promise`\<`T`\>

Defined in: [src/lib/tauri.ts:91](https://github.com/saagpatel/IncidentReview/blob/7dc21b3c9e6aea21725f6b6423572d38b0cfba7d/src/lib/tauri.ts#L91)

## Type Parameters

### T

`T`

## Parameters

### command

`string`

### args

`Record`\<`string`, `unknown`\> | `undefined`

### schema

`ZodType`\<`T`, `unknown`, `$ZodTypeInternals`\<`T`, `unknown`\>\> | `null`

## Returns

`Promise`\<`T`\>
