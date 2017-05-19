#[macro_use]
extern crate raml;

// use std::io;
// use std::io::Write;

caml!(ml_send_int, |v, v2|, <l>, {
    let x = int_val!(v);
    l = val_int!(0xbeef);
    println!("send_int  0x{:x}", x);
    // io::stdout().flush();
} -> l);

caml!(ml_send_two, |v, v2|, {
    println!("local root addr: {:p} caml_local_roots: {:#?}, v: {:?}", &raml::memory::caml_local_roots, raml::memory::caml_local_roots, v);
    let x = int_val!(v);
    let len = raml::mlvalues::caml_string_length(v2);
    let ptr = string_val!(v2);
    let slice = ::std::slice::from_raw_parts(ptr, len);
    let string = ::std::str::from_utf8_unchecked(slice);
    println!("got  0x{:x}, {}", x, string);
});
