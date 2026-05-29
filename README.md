# copper

A Scheme interpreter written in Rust with [repl_lib](https://crates.io/crates/repl_lib).

### Goal

Create an `R7RS-small` compliant Lisp interpreter. The plan is to release`v1.0.0` when copper is fully `R7RS-small` compliant.

### Features

Implemented features include:
- `define`, `lambda`, `quote`, `quasiquote`, and `if` special forms.
- Implicit conversion between numeric types (Integer, Real, Rational, Complex).
- IO Ports, `write`, `write-simple`, and `write-shared`.
- File parsing and loading.

### Inspiration
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), a small Lisp project in Rust.
- [Build your own Lisp](https://www.buildyourownlisp.com/)
