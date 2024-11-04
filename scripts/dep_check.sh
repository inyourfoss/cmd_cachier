#!/usr/bin/env sh

dep_check(){
	dep_missing=0
	for dep in "$@"
	do
		if ! command -v "$dep" > /dev/null 2>&1 ; then
			dep_missing=1
			printf "\033[1;4;33m%s\033[0m\033[33m not found\033[0m\n-> Make sure \033[1;4m%s\033[0m is installed on your system\n\n" "$dep" "$dep"
		fi
	done
	return $dep_missing
}

dep_check asciidoctor pandoc git make cargo rustc python3 \
	&& python3 -c 'import sys; sys.version_info < (3, 11) and sys.exit("Python 3.11 or newer is needed.")' \
	&& echo "All dependencies installed."

