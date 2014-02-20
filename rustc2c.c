#include <stdio.h>
#include <stdint.h>
#include <dlfcn.h>

void callee(char const* from) {
    printf("called C from %s\n", from);
}
void caller(void (*f)(char const *from)) {
    f("C");
}
void caller_foo_rs(uint8_t *ctxt);

typedef void (*callee_fcn_t)(char *from);
typedef void (*caller_fcn_t)(void (*f)(char const *from));
typedef void (*call2d_fcn_t)(void (*f)(void (*g)(char const *from)));

typedef uint32_t (*cwnr_fcn_t)(uint32_t argc, char const **argv,
                               uint8_t *ctxt, void (*f)(uint8_t *ctxt));
typedef uint32_t (*rrustc_fcn_t)(char const **argv, uint32_t argc);


int main() {
    char const *libfile = "librustc2c.dylib";
    void *handle = dlopen(libfile, RTLD_NOW);
    if (!handle) {
        printf("failed to dlopen %s\n", libfile);
    } else {
        printf("Hello World from C\n");
        char const *hello = "hello";
        void *hello_sym = dlsym(handle, hello);
        if (!hello_sym) {
            printf("No sym %s found\n", hello);
        } else {
            void (*hello_fcn)() = hello_sym;
            hello_fcn(); 
        }

        char const *mul3 = "mul3";
        char const *add5 = "add5";
        void *mul3_sym = dlsym(handle, mul3);
        void *add5_sym = dlsym(handle, add5);
        if (!mul3_sym) {
            printf("No sym %s found\n", mul3);
        }
        if (!add5_sym) {
            printf("No sym %s found\n", add5);
        }

        char const *callee_str = "callee";
        char const *caller_str = "caller";
        char const *call2d_str = "caller_2nd";
        void *callee_sym = dlsym(handle, callee_str);
        if (!callee_sym) {
            printf("No sym %s found\n", callee_str);
        }
        void *caller_sym = dlsym(handle, caller_str);
        if (!caller_sym) {
            printf("No sym %s found\n", caller_str);
        }
        void *call2d_sym = dlsym(handle, call2d_str);
        if (!call2d_sym) {
            printf("No sym %s found\n", call2d_str);
        }

        if (caller_sym && callee_sym && call2d_str) {
            callee_fcn_t callee_fcn = callee_sym;
            caller_fcn_t caller_fcn = caller_sym;
            call2d_fcn_t call2d_fcn = call2d_sym;

            callee_fcn("C");
            caller_fcn(callee);
            call2d_fcn(caller);
        }

        char const *cwnr_str     = "call_with_native_runtime";
        char const *rrustc_str = "run_compiler";

        void *cwnr_sym = dlsym(handle, cwnr_str);
        if (!cwnr_sym) {
            printf("No sym %s found\n", cwnr_str);
        }
        void *rrustc_sym = dlsym(handle, rrustc_str);
        if (!rrustc_sym) {
            printf("No sym %s found\n", rrustc_str);
        }
        if (cwnr_sym && rrustc_sym) {
            char const *args[0] = { };
            cwnr_fcn_t cwnr = (cwnr_fcn_t)cwnr_sym;
            cwnr(0, args, rrustc_sym, caller_foo_rs);
        }
    }
}

void caller_foo_rs(uint8_t *ctxt) {
    char const *args[2] = {
        "rustc", "foo.rs"
    };
    rrustc_fcn_t rrustc = (rrustc_fcn_t)ctxt;
    rrustc(args, 2);
}
