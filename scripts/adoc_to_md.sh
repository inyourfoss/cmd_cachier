
asciidoctor -b docbook --out-file - templates/templ_readme.adoc | pandoc -f docbook -t markdown_strict -o - 
