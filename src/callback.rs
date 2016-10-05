/// Callbacks from C to OCaml

use mlvalues::Value;

extern "C" {
    pub fn caml_callback (closure: Value, arg: Value) -> Value;
    pub fn caml_callback2 (closure: Value, arg1: Value, arg2: Value) -> Value;
    pub fn caml_callback3 (closure: Value, arg1: Value, arg2: Value, arg3: Value) -> Value;
    pub fn caml_callbackN (closure: Value, narg: usize, args: *mut Value) -> Value;
    pub fn caml_main (argv: *mut *mut u8);
    pub fn caml_startup (argv: *mut *mut u8);
    pub static mut caml_callback_depth: usize;
}

/*
CAMLextern value caml_callback_exn (value closure, value arg);
CAMLextern value caml_callback2_exn (value closure, value arg1, value arg2);
CAMLextern value caml_callback3_exn (value closure,
                                     value arg1, value arg2, value arg3);
CAMLextern value caml_callbackN_exn (value closure, int narg, value args[]);

#define Make_exception_result(v) ((v) | 2)
#define Is_exception_result(v) (((v) & 3) == 2)
#define Extract_exception(v) ((v) & ~3)

CAMLextern value * caml_named_value (char const * name);
typedef void (*caml_named_action) (value*, char *);
CAMLextern void caml_iterate_named_values(caml_named_action f);
*/
