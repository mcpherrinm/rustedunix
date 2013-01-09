fn seq(start: float, incr: float, end: float, sep: &str) {
  io::println(fmt!("start %f, incr %f, end %f", start, incr, end));

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


fn main() {


  seq(1f, 0.5f, 5f, ~"\n");
}
