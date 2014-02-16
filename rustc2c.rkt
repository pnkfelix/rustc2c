#lang racket

(require ffi/unsafe
         ffi/unsafe/define)

(define (locally-built-libs)
  (list (build-path (current-directory) "objdir")))

(define-ffi-definer define-rustc2c
  (ffi-lib "librustc2c" #:get-lib-dirs locally-built-libs))


(displayln "Hello World from Racket")

(define-rustc2c hello (_fun -> _void))
(hello)

(define-rustc2c mul3 (_fun _uint32 -> _uint32))
(define-rustc2c add5 (_fun _uint32 -> _uint32))

(define callee_fcn_t (_fun _string -> _void))
(define caller_fcn_t (_fun (_fun _string -> _void) -> _void))
(define call2d_fcn_t (_fun (_fun (_fun _string -> _void) -> _void) -> _void))

(define-rustc2c callee callee_fcn_t)
(define-rustc2c caller caller_fcn_t)
(define-rustc2c call2d call2d_fcn_t #:c-id caller_2nd)

(callee "Racket")
(caller (lambda (from) (displayln (string-append "called Racket from " from))))
(call2d (lambda (g) (g "Racket")))
