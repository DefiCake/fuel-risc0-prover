.PHONY: start start-relay node-hardhat node node-docker build test dev

# RISC0_BUILD_DEBUG=1 is a temporary workaround until https://github.com/risc0/risc0/pull/1257
# ETA for this fix: v0.20.0-rc2
# Meanwhile, build times will skyrocket
start:
	RISC0_BUILD_DEBUG=1 RISC0_DEV_MODE=1 RUST_LOG="executor=info" cargo run --release
start-relay:
	RISC0_DEV_MODE=1 RUST_LOG="executor=info" cargo run --bin relay
node-hardhat:
	docker-compose up hardhat
node:
	anvil --chain-id 5
node-docker:
	docker-compose up -d --no-recreate
build:
	cargo build
test:
	cargo +stable test -p test -- --nocapture
dev:
	cargo +stable watch -x "test -p test -- --nocapture"