fn seq(start: float, incr: float, end: float, sep: &str) {
  let mut i: int = 0;
  let count = (end - start)/incr as int;
  if(count < 0) {
    return;
  }

  while i <= count{
    io::print(fmt!("%f%s", start + (i as float)*incr, sep));
    i += 1;
  }
}

fn usage() {
  io::println("seq LAST");
  io::println("seq FIRST LAST");
  io::println("seq FIRST INCR LAST");
}

macro_rules! setif(
  ($setme:ident, $len:expr) => (
    if(len > $len) {
      $setme = float::from_str(args[len-$len]).get()
    }
  )
)

fn main() {
  let args = os::args();
  let sep = ~"\n";

  let mut start = 1f;
  let mut incr = 1f;
  let mut end = 1f;

  let len = args.len();
  if(len == 1) {
    usage();
    return
  }

  setif!(start, 3);
  setif!(end, 1);
  setif!(incr, 2);

  seq(start, incr, end, sep);
}
