;; Implementing a "reverse" procedure.
(define (reverse-inner ls acc)
    (if (null? ls) acc
        (reverse-inner (cdr ls) (cons (car ls) acc))))

(define (reverse my-list) (reverse-inner my-list '()))

(reverse '(1 2 3))
