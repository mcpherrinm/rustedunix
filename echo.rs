fn main() {
  let args = os::args();
  let newline = true;

  if args.len() == 0 {
    return;
  }
  io::print(args[1]);
  let mut i = 2;
  while i < args.len() {
    io::print(" ");
    io::print(args[i]);
    i+=1;
  }
  if newline {
    io::print("\n");
  }
}
