[**incidentreview**](../../README.md)

---

[incidentreview](../../README.md) / [tauri](../README.md) / invokeValidated

# Function: invokeValidated()

> **invokeValidated**\<`T`\>(`command`, `args`, `schema`): `Promise`\<`T`\>

Defined in: [src/lib/tauri.ts:91](https://github.com/saagpatel/IncidentReview/blob/f25f9d5c298ccce37f24ee3e5d5eadc4b8ed435f/src/lib/tauri.ts#L91)

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
