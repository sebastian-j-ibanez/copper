# copper
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/copper?color=orange)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/sebastian-j-ibanez/copper/rust.yml)

A Scheme interpreter written in Rust.

### Goals

Create a `R7RS` compliant Lisp interpreter to learn more about interpreters, Lisp, and Rust.

Our goal is to have `v1.0.0` be fully `R7RS` compliant.

### Status

As of `v0.2.1` features include:
- Abstract syntax tree parsing.
- File parsing and loading.
- `R7RS` numeric types (implicit conversion between them).
- Basic std lib procedures and predicates.

### Influences
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), a small Lisp project in Rust.
  - The author wrote a fantastic article about it [here](https://stopa.io/post/222).
- [Build your own Lisp](https://www.buildyourownlisp.com/)
