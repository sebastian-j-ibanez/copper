# copper
![GitHub License](https://img.shields.io/github/license/sebastian-j-ibanez/copper?color=orange)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/sebastian-j-ibanez/copper/rust.yml)

A Scheme interpreter written in Rust.

### Goals

Create a `R7RS` compliant Lisp interpreter to learn more about interpreters, Lisp, and Rust.

Our goal is to have `v1.0.0` be fully `R7RS` compliant.

### Status

As of `v0.2.2` features include:
- List processing and manipulation.
- File parsing and loading.
- Basic standard library procedures and functions.
- Implicit conversion between numeric types (Integer, Real, Rational, Complex).
- `define` and `lambda` special forms.

### REPL

<img width="500" height="auto" alt="copper-example-2 (1)" src="https://github.com/user-attachments/assets/4fa8b8f8-7b37-4c32-9670-eb2161b4a10b" />



### Influences
- [steel](https://github.com/mattwparas/steel), an embedded scheme interpreter in Rust.
- [risp](https://github.com/stopachka/risp?tab=readme-ov-file), a small Lisp project in Rust.
  - The author wrote a fantastic article about it [here](https://stopa.io/post/222).
  - The article helped me make the initial parser crate.
- [Build your own Lisp](https://www.buildyourownlisp.com/)
