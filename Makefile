##
# Project Title
#
# @file
# @version 0.1
#
.PHONY: build run

# Set the cargo command
CARGO = cargo

# Set the targets
wasm = wasm32-unknown-unknown
linux = x86_64-unknown-linux-gnu

# Set the packages
server = orb-server
runtime = orb-runtime
wasm-test = wasm-test

build-wasm:
	$(CARGO) build --package $(wasm-test) --target $(wasm)

build: build-wasm
	$(CARGO) build --package $(runtime) --target $(linux)
	$(CARGO) build --package $(server) --target $(linux)

run:
	$(CARGO) run --package $(server) --target $(linux)

watch: build-wasm
	$(CARGO) watch -s 'make build-wasm && cargo run -p $(server)' --package $(server)

clean:
	$(CARGO) clean
# end
