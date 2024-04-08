#!/usr/bin/env sh

# convert adoc to docbook as intermediate format
asciidoctor -b docbook5  "$1" --out-file - | \
    pandoc -f docbook -t markdown_strict -o -  # finally convert docbook to markdown to stdout
