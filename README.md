# Adder Compiler – Assignment 1

## Overview
This project implements a simple compiler for the Adder language.

Supported operations:
- 32-bit integers
- add1
- sub1
- negate

The compiler:
1. Parses S-expressions
2. Builds an AST
3. Generates x86-64 assembly (macho64 on macOS)
4. Links with a Rust runtime
5. Produces an executable

---

## Build Instructions

To compile and run a test:

    make test/01_37.run
    ./test/01_37.run

To regenerate transcript:

    make -B test/01_37.run
    (see transcript.txt for full demonstration)

---

## File Structure

- src/main.rs – Compiler
- runtime/start.rs – Runtime entry
- Makefile – Build system
- test/ – Test programs
- transcript.txt – Required execution transcript

---

## Notes

- Assembly target: macho64 (macOS)
- All tests respect 32-bit signed integer bounds
