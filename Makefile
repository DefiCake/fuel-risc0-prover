.PHONY: start start-relay node-hardhat node node-docker build test dev

# Use RISC0_DEV_MODE=1 RUST_LOG="executor=info" for metrics and benchmarking
start:
	cargo run --release
start-relay:
	cargo run --bin relay
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