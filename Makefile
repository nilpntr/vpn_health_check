.PHONY: all
all: build run

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	./target/debug/vpn_health_check wg0

.PHONY: release
release: build-debian

.PHONY: build-debian
build-debian:
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --release --target=x86_64-unknown-linux-gnu
