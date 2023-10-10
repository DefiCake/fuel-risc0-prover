.PHONY: start start-relay

start:
	RUST_LOG=debug cargo run --bin host
start-relay:
	RUST_LOG=debug cargo run --bin relay
node-hardhat:
	docker-compose up hardhat
node:
	anvil --chain-id 5
node-docker:
	docker-compose up -d --no-recreate
test:
	cargo +stable test -p prover-core 