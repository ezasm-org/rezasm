.DEFAULT_GOAL := help

help:
	@echo 'Available targets:'
	@echo -e 'web-dev \t\t-- run the development environment for the website'
	@echo -e 'web-build \t\t-- build the website for production'
	@echo -e 'app-build \t\t-- build the cli application'
	@echo -e 'app-run \t\t-- run the cli application'

clean:
	-rm -r ./build/
	-rm -r ./target/


app-run:
	cargo run --package rezasm --bin rezasm-app

app-build:
	cargo build --release --package rezasm --bin rezasm-app

web-dev:
	npm run tauri-dev

web-build:
	npm run tauri-build




