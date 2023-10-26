all:
	cargo build --release 
	@echo "Make sure $$HOME/.local/bin is in your PATH environment variable."
	mkdir -p "$$HOME/.local/bin/"
	cp ./target/release/rced "$$HOME"/.local/bin/cmd_cache_new
