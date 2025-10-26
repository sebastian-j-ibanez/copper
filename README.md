# copper
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/copper?color=orange)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/sebastian-j-ibanez/copper/rust.yml)

A Scheme interpreter written in Rust.

<img width="auot" height="500" alt="copper-example-2 (1)" src="https://github.com/user-attachments/assets/4fa8b8f8-7b37-4c32-9670-eb2161b4a10b" />

### Goals

Create a `R7RS` compliant Lisp interpreter to learn more about Scheme, Rust, interpreters, parsers, and tokenizers.

The goal is to have `v1.0.0` be fully `R7RS` compliant.

### Status

Implemented features include:
- List processing and manipulation.
- File parsing and loading.
- Basic standard library procedures and functions.
- Implicit conversion between numeric types (Integer, Real, Rational, Complex).
- `quote`, `define` and `lambda` special forms.

### Inspiration
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), a small Lisp project in Rust.
- [Build your own Lisp](https://www.buildyourownlisp.com/)
