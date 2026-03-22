[**incidentreview**](../../README.md)

***

[incidentreview](../../README.md) / [tauri](../README.md) / invokeValidated

# Function: invokeValidated()

> **invokeValidated**\<`T`\>(`command`, `args`, `schema`): `Promise`\<`T`\>

Defined in: [src/lib/tauri.ts:91](https://github.com/saagpatel/IncidentReview/blob/0e2d0e4b49c7d6af9d6183f418bd05b1e0fe7fe9/src/lib/tauri.ts#L91)

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
