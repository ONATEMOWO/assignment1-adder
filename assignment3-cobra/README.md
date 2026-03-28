# Cobra Compiler – Assignment 3

## Overview
This folder contains the Week 3 Cobra compiler.

Cobra extends Boa with:
- booleans
- input
- if expressions
- block expressions
- loop and break
- set! mutation
- runtime type checking
- tagged values

## Tagging Scheme
Cobra uses tagged values at runtime.

- Numbers are shifted left by 1 bit
  - example: `5` is stored as `10`
- Booleans use odd values
  - `false = 1`
  - `true = 3`

This means:
- numbers have least-significant bit `0`
- booleans have least-significant bit `1`

### Examples
- source `5` → runtime value `10`
- source `0` → runtime value `0`
- source `false` → runtime value `1`
- source `true` → runtime value `3`

## Runtime Error Codes
The runtime uses:
- `1` for `invalid argument`
- `2` for `overflow`

## Main Files
- `src/main.rs` — compiler
- `runtime/start.rs` — runtime
- `Makefile` — build rules
- `test/` — test cases
- `transcript.txt` — terminal demonstration

## Feature Examples
- booleans: `test/true_val.snek`, `test/false_val.snek`
- input: `test/input_num.snek`, `test/input_true.snek`
- arithmetic: `test/add1.snek`, `test/sub1.snek`, `test/negate.snek`
- predicates: `test/isnum_num.snek`, `test/isbool_bool.snek`
- conditionals: `test/if_true.snek`, `test/if_false.snek`
- mutation: `test/let_set.snek`
- loops: `test/loop_break.snek`
- comparisons: `test/eq_num.snek`, `test/lt_num.snek`, `test/ge_num.snek`
- errors: `test/type_error_add.snek`, `test/type_error_eq.snek`, `test/break_outside.snek`

## Notes
This assignment was started from the Week 2 Boa compiler, then extended for Week 3.
