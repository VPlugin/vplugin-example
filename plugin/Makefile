CC=clang
CFLAGS=-O3 -g0 -fno-builtin -Wall -Wextra -Wpedantic -Werror=write-strings -shared -fPIC -lGL -lGLEW
OUTFILE=plugin.obj

default: src/plugin.c
	$(CC) src/plugin.c -o $(OUTFILE) $(CFLAGS)
	vplugin-package -o plugin-example.vpl
	@echo "build: Finished"

clean: plugin.obj plugin-example.vpl
	@rm -rfv plugin.obj plugin-example.vpl