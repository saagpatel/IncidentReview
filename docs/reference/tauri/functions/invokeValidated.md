[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [tauri](../README.md) / invokeValidated

# Function: invokeValidated()

> **invokeValidated**\<`T`\>(`command`, `args`, `schema`): `Promise`\<`T`\>

Defined in: [src/lib/tauri.ts:91](https://github.com/saagpatel/IncidentReview/blob/62f6666f7da35ca351b9e7e8e3767b0ccadae45b/src/lib/tauri.ts#L91)

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
