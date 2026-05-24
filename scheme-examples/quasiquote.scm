;; Quasiquoting example
;;
;; Quasiquoting allows us to quote while still evaluating specific expressions.
;; You can quasiquote by using the "quote" procedure OR by using a backtick (`).

(println "running: (define two 2)")
(define two 2)

(println "running: `(1 ,two 3)")
`(1 ,two 3)
