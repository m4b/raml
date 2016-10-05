/** ## CAMLParam Macros
   The following macros are used to declare C local variables and
   function parameters of type [value].

   The function body must start with one of the [CAMLparam] macros.
   If the function has no parameter of type [value], use [CAMLparam0].
   If the function has 1 to 5 [value] parameters, use the corresponding
   [CAMLparam] with the parameters as arguments.
   If the function has more than 5 [value] parameters, use [CAMLparam5]
   for the first 5 parameters, and one or more calls to the [CAMLxparam]
   macros for the others.
   If the function takes an array of [value]s as argument, use
   [CAMLparamN] to declare it (or [CAMLxparamN] if you already have a
   call to [CAMLparam] for some other arguments).

   If you need local variables of type [value], declare them with one
   or more calls to the [CAMLlocal] macros at the beginning of the
   function, after the call to CAMLparam.  Use [CAMLlocalN] (at the
   beginning of the function) to declare an array of [value]s.

   Your function may raise an exception or return a [value] with the
   [CAMLreturn] macro.  Its argument is simply the [value] returned by
   your function.  Do NOT directly return a [value] with the [return]
   keyword.  If your function returns void, use [CAMLreturn0].

   All the identifiers beginning with "caml__" are reserved by OCaml.
   Do not use them for anything (local or global variables, struct or
   union tags, macros, etc.)
*/

use std::default::Default;
use std::ptr;

//#[macro_use]
use mlvalues::{Value};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CamlRootsBlock {
    pub next: *mut CamlRootsBlock,
    pub ntables: usize,
    pub nitems: usize,
    pub tables: [*mut Value; 5],
}

impl Default for CamlRootsBlock {
    fn default() -> CamlRootsBlock {
        CamlRootsBlock {
            next: ptr::null_mut(),
            ntables: 0,
            nitems: 0,
            tables: [ptr::null_mut(); 5],
        }
    }
}

/// These receive their implementations when you successfully link against an OCaml binary
extern "C" {
    pub static mut caml_local_roots: *mut CamlRootsBlock;
    pub fn caml_modify(addr: *mut Value, value: Value);
}

/// Return unit
/// ```C
/// #define CAMLreturn0 do{ \
/// caml_local_roots = caml__frame; \
/// return; \
/// }while (0)
/// ```
#[macro_export]
macro_rules! return0 {
  () => (caml_local_roots = caml_frame; return);
}

#[macro_export]
macro_rules! store_field {
    ($block: ident, $offset: expr, $val: ident, ) => (
      caml_modify (&field!($block, $offset), $val);
    );
}

/// Stores the `value` in the `block` at `offset`.
/// Original C code:
/// ```C
/// Store_field(block, offset, val) do{ \
/// mlsize_t caml__temp_offset = (offset); \
/// value caml__temp_val = (val); \
/// caml_modify (&Field ((block), caml__temp_offset), caml__temp_val); \
/// }while(0)
/// ```
pub unsafe fn store_field (block: *mut Value, offset: usize, value: Value) {
    let contents = (block.offset(offset as isize)) as *mut Value;
    caml_modify(contents, value);
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! caml_param {

    (@step $idx:expr, $caml_roots:ident,) => {
        $caml_roots.ntables = $idx;
    };

    (@step $idx:expr, $caml_roots:ident, $param:ident, $($tail:ident,)*) => {
        $caml_roots.tables[$idx] = &mut $param;
        caml_param!(@step $idx + 1usize, $caml_roots, $($tail,)*);
    };

    ($($n:ident),*) => {
        let mut caml_roots: $crate::memory::CamlRootsBlock = ::std::default::Default::default();
        caml_roots.next = $crate::memory::caml_local_roots;
        $crate::memory::caml_local_roots = (&mut caml_roots) as *mut $crate::memory::CamlRootsBlock;
        caml_roots.nitems = 1; // this is = N when CAMLxparamN is used
        caml_param!(@step 0usize, caml_roots, $($n,)*);
    }
}

#[macro_export]
macro_rules! caml_ffi {
//, $($n:ident),*

    ($code:tt) => {
        let mut caml_frame = $crate::memory::caml_local_roots.clone();
        $code;
//        $crate::memory::caml_local_roots = caml_frame;
        return
    };

    ($code:tt => $result:ident) => {
        let mut caml_frame = $crate::memory::caml_local_roots;
//        caml_param!($(n,)*);
        $code;
//        unsafe { $crate::memory::caml_local_roots = caml_frame;}
        return $result;
//        caml_return!(caml_frame, $result);
    }
}

/// Initializes and registers the given identifier(s) as a local value with the OCaml runtime.
/// C code:
/// ```c
/// #define CAMLlocal1(x) \
/// value x = Val_unit; \
/// CAMLxparam1 (x)
/// ```
#[macro_export]
macro_rules! caml_local {
    ($($local:ident),*) => {
        $(let mut $local = $crate::mlvalues::UNIT;)*
        caml_param!($($local),*);
    }
}

#[macro_export]
macro_rules! caml_body {

    (|$($param:ident),*|, @code $code:block) => {
        let caml_frame = $crate::memory::caml_local_roots;
        caml_param!($($param),*);
        $code;
        $crate::memory::caml_local_roots = caml_frame;
    };

    (|$($param:ident),*|, <$($local:ident),*>, $code:block) => {
        let caml_frame = $crate::memory::caml_local_roots;
        caml_param!($($param),*);
        caml_local!($($local),*);
        $code;
        $crate::memory::caml_local_roots = caml_frame;
    }
}

#[macro_export]
macro_rules! caml {

    ($name:ident, |$($param:ident),*|, <$($local:ident),*>, $code:block -> $retval:ident) => {
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::mlvalues::Value,)*) -> $crate::mlvalues::Value {
            caml_body!(|$($param),*|, <$($local),*>, $code);
            return $retval;
        }
    };

    ($name:ident, |$($param:ident),*|, $code:block) => {
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::mlvalues::Value,)*) {
            caml_body!(|$($param),*|, @code $code);
            return;
        }
    };

    ($name:ident, |$($param:ident),*|, $code:block -> $retval:ident) => {
        #[no_mangle]
        pub unsafe extern fn $name ($(mut $param: $crate::mlvalues::Value,)*) -> $crate::mlvalues::Value {
            caml_body!(|$($param),*|, @code $code);
            return $retval;
        }
    };

}

#[cfg(test)]
mod test {

    use super::super::mlvalues::Value;

    #[cfg(test)]
    fn test_paramx() {
        let v: Value = 0x0;
//        caml_param!(v);
        assert!(true);
    }

}
