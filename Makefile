MAN_PAGE_INSTALL_DIR = /usr/share/man
SYSTEM_BIN_DIR = /usr/bin
LOCAL_BIN_DIR = $$HOME/.local/bin


RELEASE_BUILD_CMD = "cd crate && cargo build --release"
RELEASE_BIN = ./crate/target/release/cmd_cachier
MAN_PAGE_DIR = ./docs/manpages

build:
	sh -c $(RELEASE_BUILD_CMD)

install:
	@echo "Make sure $(LOCAL_BIN_DIR) is in your PATH environment variable."
	mkdir -p $(LOCAL_BIN_DIR)
	cp $(RELEASE_BIN) "$(LOCAL_BIN_DIR)/cmd_cachier"

uninstall:
	rm -f "$(LOCAL_BIN_DIR)/cmd_cachier" 

systeminstall:
	mkdir -p $(MAN_PAGE_INSTALL_DIR)/man1
	cp $(RELEASE_BIN) "$(SYSTEM_BIN_DIR)"
	cp $(MAN_PAGE_DIR)/cmd_cachier.1 "$(MAN_PAGE_INSTALL_DIR)/man1"
	mandb

systemuninstall:
	rm -f "$(SYSTEM_BIN_DIR)/cmd_cachier"
	rm -f "$(MAN_PAGE_INSTALL_DIR)/man1/cmd_cachier*"

untempl:
	scripts/untempl.py

readme: untempl
	scripts/adoc_to_md.sh "docs/README.adoc" > README.md

manpage_pdf: untempl
	asciidoctor-pdf -b pdf $(MAN_PAGE_DIR)/manpages/cmd_cachier.1.adoc \
		--out-file docs/manpages/cmd_cachier.1.pdf

manpage_man: untempl
	asciidoctor -b manpage $(MAN_PAGE_DIR)/cmd_cachier.1.adoc \
		--out-file docs/manpages/cmd_cachier.1

manpage_html: untempl
	asciidoctor -b html5 $(MAN_PAGE_DIR)/cmd_cachier.1.adoc \
		--out-file docs/manpages/cmd_cachier.1.html

documentation: manpage_pdf manpage_man manpage_html readme
	@echo created documentation

release: build manpage_man manpage_html readme
	sh -c "$(RELEASE_BUILD_CMD)"

clean:
	scripts/clean.sh

