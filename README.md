# copper
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/sebastian-j-ibanez/copper/rust.yml)
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/copper?color=E57300)

A Scheme interpreter written in Rust with [repl_lib](https://crates.io/crates/repl_lib).

### Goal

Create an `R7RS-small` compliant Lisp interpreter. The plan is to release`v1.0.0` when copper is fully `R7RS-small` compliant.

### Status

Implemented features include:
- List processing and manipulation.
- File parsing and loading.
- Implicit conversion between numeric types (Integer, Real, Rational, Complex).
- `if`, `quote`, `define` and `lambda` special forms.

### Inspiration
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), a small Lisp project in Rust.
- [Build your own Lisp](https://www.buildyourownlisp.com/)
