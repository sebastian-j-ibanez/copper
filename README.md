# copper
[![Rust](https://img.shields.io/badge/Rust-orange.svg?e&logo=rust&logoColor=white)](#)
[![Lisp](https://img.shields.io/badge/R7RS-1f6cb0.svg?e&logo=commonlisp&logoColor=white&labelColor=1f6cb0)](#)
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/copper?color=brown)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/sebastian-j-ibanez/copper/rust.yml)

A Lisp interpreter written in Rust.

### Status 🚧

Copper is in active development and is not `R7RS-small` compliant yet. 

As of 0.1.0 features include:
- Abstract syntax tree parsing
- Basic type checking
- `+`, `-`, `*`, `/` functions

### Influences
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), another Lisp project in Rust.
  - The author wrote a fantastic article about it [here](https://stopa.io/post/222).
- [Build your own Lisp](https://www.buildyourownlisp.com/)

### Goals

Create a `R7RS-small` compliant Lisp interpreter to learn more about Lisp, interpreters, and Rust.
