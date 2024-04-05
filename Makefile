build:
	cargo build --release 
run-build:
	Makescripts/run-build.sh
install:
	@echo "Make sure $$HOME/.local/bin is in your PATH environment variable."
	mkdir -p "$$HOME/.local/bin/"
	cp ./target/release/cmd_cachier "$$HOME"/.local/bin/cmd_cachier
uninstall:
	@echo "Make sure $$HOME/.local/bin is in your PATH environment variable."
	mkdir -p "$$HOME/.local/bin/"
	cp ./target/release/cmd_cachier "$$HOME"/.local/bin/cmd_cachier
systeminstall:
	mkdir -p /usr/share/man/man1
	cp ./target/release/cmd_cachier "/usr/bin/"
	cp ./man/man1/cmd_cachier.1 "/usr/share/man/man1"
	mandb
systemuninstall:
	rm -f "/usr/bin/cmd_cachier"
	rm -f "/usr/share/man/man1/cmd_cachier*"
documentation:
	python3 scripts/prep_docs.py > docs/cmd_cachier.1.adoc
	asciidoctor-pdf -b pdf docs/cmd_cachier.1.adoc --out-file docs/cmd_cachier.1.pdf
	asciidoctor -b manpage docs/cmd_cachier.1.adoc --out-file man/man1/cmd_cachier.1
	asciidoctor -b html5 docs/cmd_cachier.1.adoc --out-file docs/cmd_cachier.1.html
prep-release:
	python3 scripts/prep_release.py > default.nix
	
