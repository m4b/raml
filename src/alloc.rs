use mlvalues::{Mlsize_t,Value, Tag_t};

extern {
  pub fn caml_alloc (size: Mlsize_t, tag: Tag_t) -> Value;
  pub fn caml_alloc_small (size: Mlsize_t, tag: Tag_t) -> Value;
  pub fn caml_alloc_tuple (size: Mlsize_t) -> Value;
  pub fn caml_alloc_string (size: Mlsize_t) -> Value; // size in bytes
  pub fn caml_copy_string (string: *const u8) -> Value;
  pub fn caml_copy_string_array (arr: *const *const u8) -> Value;
  /* TODO: add these
  pub fn caml_copy_double (double) -> Value;
  pub fn caml_copy_int32 (int32) -> Value/ ; // defined in [ints.c]
  pub fn caml_copy_int64 (int64) -> Value/; // defined in [ints.c]
  pub fn caml_copy_nativeint (intnat) -> Value/; // defined in [ints.c]
  pub fn caml_alloc_array (value ( * funct) (char const *), char const * * array) -> Value;
  */
// CAMLextern value caml_alloc_sprintf( const char * format, ... ); // this is going to be interesting
}