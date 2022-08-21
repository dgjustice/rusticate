SHELL := /bin/bash

.PHONY: all build clean lint test

all: clean lint test

build:
	cargo build

clean:
	cargo clean

lint:
	cargo fmt --check
	cargo clippy

test:
	cargo test
