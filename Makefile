.PHONY: build-parser build-api build-web clean

build-parser:
	cd parser-core && cargo build --release

build-api: build-parser
	cd api-gateway && go build -o ../bin/server ./cmd/server

build-web:
	cd web && npm run build

build-all: build-api build-web

clean:
	rm -rf bin/*
	cd parser-core && cargo clean
	cd web && rm -rf .svelte-kit build node_modules