fn do_cat(in: io::Reader) {
  let mut buf: [u8 * 4096] = [0, ..4096];
  let out = io::stdout();
  loop {
    if in.eof() {
      return;
    }
    let n = in.read(buf, 4096);
    out.write(buf.view(0, n));
  }
}


fn main() {
  let argv = os::args();
  let files = argv.view(1, argv.len());
  if !files.is_empty() {
    let input = files.map(|f| {
      if(*f == ~"-") {
        io::stdin()
      } else {
        result::unwrap(io::file_reader(&path::Path(*f)))
      }
    });
    for input.each |in| {
      do_cat(*in);
    }
  } else {
    do_cat(io::stdin());
  }

}
