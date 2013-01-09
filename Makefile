default: hello cat seq

hello: hello.rs
	rustc hello.rs

cat: cat.rs
	rustc cat.rs

seq: seq.rs
	rustc seq.rs

clean:
	-rm hello cat seq
