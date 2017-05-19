external send_int : int -> int = "ml_send_int"
external send_two : int -> string -> unit = "ml_send_two"

let f x = x land 0x0000ffff

let _ =
  let string = "string thing" in
  let deadbeef = 0xdeadbeef in
  let res = send_int 0xb1b1eb0b in
  Printf.printf "send_int returned: 0x%x\n" res;
  flush stdout;
  send_two deadbeef string;
  send_two (f deadbeef) string
