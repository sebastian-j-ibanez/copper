# copper
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/copper?color=orange)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/sebastian-j-ibanez/copper/rust.yml)

A Scheme interpreter written in Rust.

### Goals

Create a `R7RS` compliant Lisp interpreter to learn more about interpreters, Lisp, and Rust.

Our goal is to have `v1.0.0` be fully `R7RS` compliant.

### Status

As of `v0.2.1` features include:
- `define` and `lambda` macros.
- File parsing and loading.
- Basic standard library procedures and functions.
- Implicit conversion between numeric types (Integer, Real, Rational, Complex).

### Example

<img width="500" height="auto" alt="copper-example" src="https://github.com/user-attachments/assets/55b2d998-3e01-4215-a6c1-601415c1a550" />

### Influences
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), a small Lisp project in Rust.
  - The author wrote a fantastic article about it [here](https://stopa.io/post/222).
- [Build your own Lisp](https://www.buildyourownlisp.com/)
