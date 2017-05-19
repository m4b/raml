#[macro_use]
extern crate raml;

//use raml::mlvalues::*;
//use raml::memory::{CamlRootsBlock, caml_local_roots};

#[test]
fn test_memory() {
    let caml_int = 7471857119 as usize; // if this is not cast, then it's a u32 and conversion will fail
    let x = int_val!(caml_int);
    assert_eq!(x, 0xdeadbeef);
}

// #[test]
// fn test_paramx() {
//     let mut v: Value = 0x0;
//     unsafe { caml_param!(v); }
//     assert!(true);
// }
