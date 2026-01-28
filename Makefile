.PHONY: build-rust build-go build clean sync

# Targets
build-rust:
	@echo "==> Building Rust Parser..."
	cd rust-analyzer && cargo build --release
	mkdir -p bin
	cp rust-analyzer/target/release/rust-analyzer bin/parser

build-go:
	@echo "==> Building Go Backend..."
	mkdir -p bin
	go build -o bin/server cmd/server/main.go

build: build-rust build-go

clean:
	@echo "==> Cleaning artifacts..."
	rm -rf bin
	cd rust-analyzer && cargo clean

sync:
	@echo "==> Syncing environments..."
	git pull origin main
	go mod tidy