.PHONY: all build run clean test

all: build

build:
	cargo bootimage

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin -serial mon:stdio -display none

run-gui: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-operating_system_rust.bin

clean:
	cargo clean

test:
	cargo test
