# R7RS Standard Library Progress

Tracks status of all procedures and syntax forms defined in the [Revised⁷ Report on the Algorithmic Language Scheme](https://small.r7rs.org/attachment/r7rs.pdf).

## Legend

| Symbol    | Meaning |
| --------- | ------- |
| `yes`     | Implemented |
| `partial` | Partial or non-conformant |
| `no`      | Not implemented |

---

## `(scheme base)` — Base Library

### Special Forms / Syntax

| Form | Status | Notes |
| ---- | ------ | ----- |
| `define` | yes | `macros::define` |
| `lambda` | yes | `macros::lambda` — fixed-arity only; variadic rest args not supported |
| `if` | yes | `macros::if_statement` |
| `begin` | yes | `macros::begin` |
| `quote` | yes | `macros::quote` |
| `quasiquote` | yes | `macros::quasiquote` |
| `set!` | no | |
| `cond` | partial | `macros::cond` — only two-element `(test result)` clauses; no `else`, `=>`, single-test, or multi-expr bodies |
| `case` | no | |
| `and` | partial | Registered as a procedure — all args evaluated before call; no short-circuit |
| `or` | partial | Same as `and` |
| `when` | no | |
| `unless` | no | |
| `let` | yes | |
| `let*` | yes | |
| `letrec` | no | |
| `letrec*` | no | |
| `let-values` | no | |
| `let*-values` | no | |
| `define-values` | no | |
| `define-record-type` | no | |
| `define-syntax` | no | |
| `let-syntax` | no | |
| `letrec-syntax` | no | |
| `syntax-rules` | no | |
| `syntax-error` | no | |
| `parameterize` | yes | `macros::parameterize` |
| `guard` | no | |
| `include` | no | |
| `include-ci` | no | |
| `cond-expand` | no | |
| `do` | no | |

---

### 6.1 Equivalence Predicates

| Procedure | Status | Notes |
| --------- | ------ | ----- |
| `eqv?` | yes | `procedures::are_eqv` |
| `eq?` | yes | Aliased to `eqv?` |
| `equal?` | yes | `procedures::are_equal` |

---

### 6.2 Numbers

#### Type Predicates

| Procedure | Status |
| --------- | ------ |
| `number?` | yes |
| `complex?` | yes |
| `real?` | yes |
| `rational?` | yes |
| `integer?` | yes |
| `exact?` | yes |
| `inexact?` | yes |
| `exact-integer?` | yes |
| `zero?` | no |
| `positive?` | no |
| `negative?` | no |
| `odd?` | yes |
| `even?` | yes |

#### Comparison

| Procedure | Status |
| --------- | ------ |
| `=` | no |
| `<` | no |
| `>` | no |
| `<=` | no |
| `>=` | no |

#### Arithmetic

| Procedure | Status | Notes |
| --------- | ------ | ----- |
| `+` | yes | |
| `-` | yes | |
| `*` | yes | |
| `/` | yes | |
| `abs` | yes | |
| `min` | yes | |
| `max` | yes | |
| `floor` | yes | |
| `ceiling` | yes | |
| `truncate` | no | |
| `round` | no | |
| `modulo` | yes | Equivalent to `floor-remainder` |
| `quotient` | no | Equivalent to `truncate-quotient` |
| `remainder` | no | Equivalent to `truncate-remainder` |
| `floor/` | no | |
| `floor-quotient` | no | |
| `floor-remainder` | no | |
| `truncate/` | no | |
| `truncate-quotient` | no | |
| `truncate-remainder` | no | |
| `gcd` | no | |
| `lcm` | no | |
| `numerator` | no | |
| `denominator` | no | |
| `rationalize` | no | |
| `square` | no | |
| `expt` | yes | |
| `exact-integer-sqrt` | no | |
| `exact` | no | |
| `inexact` | no | |
| `number->string` | yes | |
| `string->number` | yes | |

---

### 6.3 Booleans

| Procedure | Status |
| --------- | ------ |
| `not` | yes |
| `boolean?` | yes |
| `boolean=?` | no |

---

### 6.4 Pairs and Lists

| Procedure | Status |
| --------- | ------ |
| `cons` | yes |
| `car` | yes |
| `cdr` | yes |
| `set-car!` | yes |
| `set-cdr!` | yes |
| `null?` | yes |
| `pair?` | yes |
| `list?` | yes |
| `list` | yes |
| `make-list` | no |
| `length` | yes |
| `append` | yes |
| `reverse` | yes |
| `list-tail` | no |
| `list-ref` | no |
| `list-set!` | no |
| `list-copy` | no |
| `memq` | no |
| `memv` | no |
| `member` | no |
| `assq` | no |
| `assv` | no |
| `assoc` | no |
| `caar` | yes |
| `cadr` | yes |
| `cdar` | yes |
| `cddr` | yes |

---

### 6.5 Symbols

| Procedure | Status |
| --------- | ------ |
| `symbol?` | yes |
| `symbol=?` | no |
| `symbol->string` | yes |
| `string->symbol` | yes |

---

### 6.6 Characters

| Procedure | Status |
| --------- | ------ |
| `char?` | yes |
| `char=?` | no |
| `char<?` | no |
| `char>?` | no |
| `char<=?` | no |
| `char>=?` | no |
| `char->integer` | no |
| `integer->char` | no |
| `char-alphabetic?` | yes |
| `char-numeric?` | yes |
| `char-whitespace?` | yes |
| `char-upper-case?` | yes |
| `char-lower-case?` | yes |

---

### 6.7 Strings

| Procedure | Status |
| --------- | ------ |
| `string?` | yes |
| `make-string` | no |
| `string` | yes |
| `string-length` | yes |
| `string-ref` | no |
| `string-set!` | no |
| `string=?` | no |
| `string<?` | no |
| `string>?` | no |
| `string<=?` | no |
| `string>=?` | no |
| `string-append` | yes |
| `string-upcase` | yes |
| `string-downcase` | yes |
| `substring` | no |
| `string-copy` | no |
| `string-copy!` | no |
| `string-fill!` | no |
| `string->list` | yes |
| `list->string` | yes |
| `string->vector` | yes |
| `vector->string` | yes |
| `string->utf8` | yes |
| `utf8->string` | yes |

---

### 6.8 Vectors

| Procedure | Status |
| --------- | ------ |
| `vector?` | yes |
| `make-vector` | yes |
| `vector` | yes |
| `vector-length` | yes |
| `vector-ref` | yes |
| `vector-set!` | yes |
| `vector->list` | yes |
| `list->vector` | yes |
| `vector-copy` | yes |
| `vector-copy!` | yes |
| `vector-fill!` | yes |
| `vector-append` | yes |
| `vector-map` | no |
| `vector-for-each` | no |

---

### 6.9 Bytevectors

| Procedure | Status |
| --------- | ------ |
| `bytevector?` | yes |
| `make-bytevector` | yes |
| `bytevector` | yes |
| `bytevector-length` | yes |
| `bytevector-u8-ref` | yes |
| `bytevector-u8-set!` | yes |
| `bytevector-copy` | yes |
| `bytevector-copy!` | yes |
| `bytevector-append` | yes |

---

### 6.10 Control Features

| Procedure | Status |
| --------- | ------ |
| `procedure?` | yes |
| `apply` | no |
| `map` | no |
| `string-map` | no |
| `vector-map` | no |
| `for-each` | no |
| `string-for-each` | no |
| `vector-for-each` | no |
| `call-with-current-continuation` | no |
| `call/cc` | no |
| `values` | no |
| `call-with-values` | no |
| `dynamic-wind` | no |

---

### 6.11 Exceptions

| Procedure | Status |
| --------- | ------ |
| `with-exception-handler` | no |
| `raise` | no |
| `raise-continuable` | no |
| `error` | yes |
| `error-object?` | no |
| `error-object-message` | no |
| `error-object-irritants` | no |
| `read-error?` | no |
| `file-error?` | no |

---

### 6.13 Input and Output

#### Ports

| Procedure | Status | Notes |
| --------- | ------ | ----- |
| `port?` | no | |
| `input-port?` | yes | |
| `output-port?` | yes | |
| `textual-port?` | yes | |
| `binary-port?` | yes | |
| `input-port-open?` | yes | |
| `output-port-open?` | yes | |
| `current-input-port` | yes | Implemented as a parameter |
| `current-output-port` | yes | Implemented as a parameter |
| `current-error-port` | no | |
| `close-port` | yes | |
| `close-input-port` | no | |
| `close-output-port` | no | |
| `open-input-string` | yes | |
| `open-output-string` | yes | |
| `get-output-string` | yes | |
| `open-input-bytevector` | yes | |
| `open-output-bytevector` | yes | |
| `get-output-bytevector` | yes | |
| `call-with-port` | yes | |

#### Input

| Procedure | Status |
| --------- | ------ |
| `read-char` | yes |
| `peek-char` | yes |
| `read-line` | yes |
| `read-string` | yes |
| `read-u8` | yes |
| `peek-u8` | yes |
| `read-bytevector` | yes |
| `read-bytevector!` | yes |
| `char-ready?` | yes |
| `u8-ready?` | yes |
| `eof-object?` | yes |
| `eof-object` | yes |

#### Output

| Procedure | Status |
| --------- | ------ |
| `write-char` | yes |
| `write-string` | yes |
| `write-u8` | yes |
| `write-bytevector` | no |
| `newline` | yes |
| `flush-output-port` | yes |

---

### 6.14 System Interface

| Procedure | Status |
| --------- | ------ |
| `features` | no |

---

### Dynamic Bindings

| Procedure | Status |
| --------- | ------ |
| `make-parameter` | yes |
| `parameter?` | yes |

---

## `(scheme case-lambda)` — Case-Lambda Library

| Form | Status |
| ---- | ------ |
| `case-lambda` | no |

---

## `(scheme char)` — Char Library

| Procedure | Status |
| --------- | ------ |
| `char-alphabetic?` | yes |
| `char-numeric?` | yes |
| `char-whitespace?` | yes |
| `char-upper-case?` | yes |
| `char-lower-case?` | yes |
| `char-ci=?` | no |
| `char-ci<?` | no |
| `char-ci>?` | no |
| `char-ci<=?` | no |
| `char-ci>=?` | no |
| `char-upcase` | no |
| `char-downcase` | no |
| `char-foldcase` | no |
| `digit-value` | no |
| `string-upcase` | yes |
| `string-downcase` | yes |
| `string-foldcase` | no |
| `string-ci=?` | no |
| `string-ci<?` | no |
| `string-ci>?` | no |
| `string-ci<=?` | no |
| `string-ci>=?` | no |

---

## `(scheme complex)` — Complex Library

| Procedure | Status |
| --------- | ------ |
| `make-rectangular` | no |
| `make-polar` | no |
| `real-part` | no |
| `imag-part` | no |
| `magnitude` | no |
| `angle` | no |

---

## `(scheme cxr)` — CxR Library

All 24 three- and four-deep compositions of `car`/`cdr` are implemented.

| Procedure | Status | Procedure | Status |
| --------- | ------ | --------- | ------ |
| `caaar` | yes | `cdaar` | yes |
| `caadr` | yes | `cdadr` | yes |
| `cadar` | yes | `cddar` | yes |
| `caddr` | yes | `cdddr` | yes |
| `caaaar` | yes | `cdaaar` | yes |
| `caaadr` | yes | `cdaadr` | yes |
| `caadar` | yes | `cdadar` | yes |
| `caaddr` | yes | `cdaddr` | yes |
| `cadaar` | yes | `cddaar` | yes |
| `cadadr` | yes | `cddadr` | yes |
| `caddar` | yes | `cdddar` | yes |
| `cadddr` | yes | `cddddr` | yes |

---

## `(scheme eval)` — Eval Library

| Procedure | Status |
| --------- | ------ |
| `eval` | no |
| `environment` | no |

---

## `(scheme file)` — File Library

| Procedure | Status |
| --------- | ------ |
| `open-input-file` | yes |
| `open-output-file` | yes |
| `open-binary-input-file` | yes |
| `open-binary-output-file` | yes |
| `call-with-input-file` | yes |
| `call-with-output-file` | yes |
| `with-input-from-file` | no |
| `with-output-to-file` | no |
| `file-exists?` | yes |
| `delete-file` | yes |

---

## `(scheme inexact)` — Inexact Library

| Procedure | Status |
| --------- | ------ |
| `exp` | no |
| `log` | no |
| `sin` | no |
| `cos` | no |
| `tan` | no |
| `asin` | no |
| `acos` | no |
| `atan` | no |
| `sqrt` | no |
| `finite?` | no |
| `infinite?` | no |
| `nan?` | no |

---

## `(scheme lazy)` — Lazy Library

| Form / Procedure | Status |
| ---------------- | ------ |
| `delay` | no |
| `delay-force` | no |
| `force` | no |
| `make-promise` | no |
| `promise?` | no |

---

## `(scheme load)` — Load Library

| Procedure | Status |
| --------- | ------ |
| `load` | yes |

---

## `(scheme process-context)` — Process-Context Library

| Procedure | Status |
| --------- | ------ |
| `exit` | yes |
| `emergency-exit` | no |
| `command-line` | no |
| `get-environment-variable` | no |
| `get-environment-variables` | no |

---

## `(scheme read)` — Read Library

| Procedure | Status |
| --------- | ------ |
| `read` | no |

---

## `(scheme repl)` — REPL Library

| Procedure | Status |
| --------- | ------ |
| `interaction-environment` | no |

---

## `(scheme time)` — Time Library

| Procedure | Status |
| --------- | ------ |
| `current-second` | no |
| `current-jiffy` | no |
| `jiffies-per-second` | no |

---

## `(scheme write)` — Write Library

| Procedure | Status |
| --------- | ------ |
| `write` | yes |
| `write-shared` | yes |
| `write-simple` | yes |
| `display` | yes |

---

## Non-Standard Extensions

Procedures implemented in Copper but not defined by R7RS.

| Procedure | Notes |
| --------- | ----- |
| `print` | Alias for `display` |
| `println` | `display` followed by a newline |
| `pp` | Pretty-print |
| `string-reverse` | Reverses a string |

---

## Summary

| Library | yes | partial | no |
| ------- | -- | -- | -- |
| `(scheme base)` — special forms | 8 | 3 | 20 |
| `(scheme base)` — equivalence | 3 | 0 | 0 |
| `(scheme base)` — numbers | 15 | 0 | 21 |
| `(scheme base)` — booleans | 2 | 0 | 1 |
| `(scheme base)` — pairs & lists | 12 | 0 | 12 |
| `(scheme base)` — symbols | 3 | 0 | 1 |
| `(scheme base)` — characters | 5 | 0 | 7 |
| `(scheme base)` — strings | 12 | 0 | 10 |
| `(scheme base)` — vectors | 12 | 0 | 2 |
| `(scheme base)` — bytevectors | 9 | 0 | 0 |
| `(scheme base)` — control | 1 | 0 | 12 |
| `(scheme base)` — exceptions | 1 | 0 | 8 |
| `(scheme base)` — I/O | 22 | 0 | 5 |
| `(scheme base)` — misc | 2 | 0 | 2 |
| `(scheme case-lambda)` | 0 | 0 | 1 |
| `(scheme char)` | 7 | 0 | 15 |
| `(scheme complex)` | 0 | 0 | 6 |
| `(scheme cxr)` | 24 | 0 | 0 |
| `(scheme eval)` | 0 | 0 | 2 |
| `(scheme file)` | 6 | 0 | 4 |
| `(scheme inexact)` | 0 | 0 | 12 |
| `(scheme lazy)` | 0 | 0 | 5 |
| `(scheme load)` | 1 | 0 | 0 |
| `(scheme process-context)` | 1 | 0 | 4 |
| `(scheme read)` | 0 | 0 | 1 |
| `(scheme repl)` | 0 | 0 | 1 |
| `(scheme time)` | 0 | 0 | 3 |
| `(scheme write)` | 4 | 0 | 0 |
| **Total** | **150** | **3** | **155** |
