
src=src/*.rs

all: create

run: target/x86_64-asmos/debug/bootimage-asmos.bin
	bootimage run
	#qemu-system-x86_64 -drive format=raw,file=target/x86_64-asmos/debug/bootimage-asmos.bin

create: target/x86_64-asmos/debug/bootimage-asmos.bin

target/x86_64-asmos/debug/bootimage-asmos.bin: target/x86_64-asmos/debug/asmos
	bootimage build

build: target/x86_64-asmos/debug/asmos

target/x86_64-asmos/debug/asmos: $(src)
	cargo +nightly xbuild --target "x86_64-asmos.json"

clean:
	cargo clean

test: create
	cargo +nightly xtest --target "x86_64-asmos.json"

check:
	cargo +nightly check

toolchain:
	rustup install nightly
	rustup component add llvm-tools-preview
	cargo install cargo-xbuild
	cargo install bootimage

