#lang racket

(require ffi/unsafe
         ffi/unsafe/define)

(require (only-in racket/runtime-path define-runtime-path))

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

(define-rustc2c call_with_native_runtime (_fun _uint32 (_list i _string*/utf-8) (_fun -> _void) -> _int32))
(define-rustc2c run_compiler (_fun (_list i _string*/utf-8) _uint32 -> _uint32))

(define-runtime-path foo.rs "foo.rs")

(call_with_native_runtime
 0 '()
 (lambda ()
   (displayln "Got to Racket from Rust call_with_native_runtime")
   (displayln "Running rustc now")
   (run_compiler `("rustc" ,foo.rs) 2)
   (displayln "Got back from rustc though who knows if it failed")
   ))


