CC65_TARGET = c64
LDFLAGS = -t $(CC65_TARGET)
CC = /home/srwalter/src/cc65/bin/cl65
MRUSTC ?= /home/srwalter/src/mrustc/bin/mrustc -L /home/srwalter/src/mrustc/output/

all: test.prg

test.prg: c64.c liblib.hir.o.c
	$(CC) $(LDFLAGS) -o $@ $^

c64.c: liblib.hir src/bin/c64.rs
	$(MRUSTC) --extern c64_rust=liblib.hir src/bin/c64.rs

liblib.hir liblib.hir.o.c: src/lib.rs
	$(MRUSTC) --crate-type rlib src/lib.rs

clean:
	rm test.prg *.c *.o *.hir
