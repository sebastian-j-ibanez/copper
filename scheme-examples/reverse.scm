;; Implementing a "reverse" procedure.
(define (my-reverse ls)
  (begin
    (define (my-reverse-inner ls acc)
      (if (null? ls) acc
          (my-reverse-inner (cdr ls) (cons (car ls) acc))))
    (my-reverse-inner ls '())))

(my-reverse '(1 2 3))
