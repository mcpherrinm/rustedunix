default: hello cat seq echo

hello: hello.rs
	rustc hello.rs

cat: cat.rs
	rustc cat.rs

echo: echo.rs
	rustc echo.rs

seq: seq.rs
	rustc seq.rs

clean:
	-rm hello cat seq echo
