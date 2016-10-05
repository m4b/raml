
#[macro_use] pub mod mlvalues;
#[macro_use] pub mod memory;
#[macro_use] pub mod alloc;

#[cfg(test)]
mod tests {

    #[macro_use]
    use super::mlvalues::*;

    #[macro_use]
    use super::memory::{CamlRootsBlock, caml_local_roots};

    #[test]
    fn test_memory() {
        let mut caml_int = 7471857119 as usize; // if this is not cast, then it's a u32 and conversion will fail
        //param!(caml_int);
        let x = int_val!(caml_int);
        assert_eq!(x, 0xdeadbeef);
    }
}
