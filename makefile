.PHONY: test clean format slides
test: src/*.rs
	cargo test
clean:
	cargo clean
format:
	cargo fmt
slides:
	slides/build.sh
