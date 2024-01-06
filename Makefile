.PHONY: start start-relay

# is a temporary workaround until https://github.com/risc0/risc0/pull/1257
# ETA for this fix: v0.20.0-rc2
# Meanwhile, build times will skyrocket
start:
	RISC0_DEV_MODE=1 RUST_LOG="executor=info" cargo run --release
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
	cargo +stable test -p prover-core 