build:
	cargo build --release 
install:
	@echo "Make sure $$HOME/.local/bin is in your PATH environment variable."
	mkdir -p "$$HOME/.local/bin/"
	cp ./target/release/cmd_cachier "$$HOME"/.local/bin/cmd_cachier
