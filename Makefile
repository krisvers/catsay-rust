.PHONY: all build install

all: build

build:
	cargo build -r

install:
	cp ./target/release/catsay-rust /usr/bin/
