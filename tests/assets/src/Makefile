ASSETS_DIR := ../generated

CC := gcc
AS := as
LD := ld

.PHONY: all clean

all: \
	$(ASSETS_DIR)/hello \
	$(ASSETS_DIR)/hello.o \
	$(ASSETS_DIR)/libhello.so \
	$(ASSETS_DIR)/rwx \
	$(ASSETS_DIR)/hello_pie_partial_relro \
	$(ASSETS_DIR)/hello_nopie_full_relro \
	$(ASSETS_DIR)/hello_execstack \
	$(ASSETS_DIR)/hello_norelro

$(ASSETS_DIR)/hello: hello.c
	$(CC) hello.c -no-pie -o $(ASSETS_DIR)/hello

$(ASSETS_DIR)/hello.o: hello.c
	$(CC) -c hello.c -o $(ASSETS_DIR)/hello.o

$(ASSETS_DIR)/libhello.so: libhello.c
	$(CC) -shared -fPIC libhello.c -o $(ASSETS_DIR)/libhello.so

$(ASSETS_DIR)/rwx: rwx.s
	$(AS) rwx.s -o $(ASSETS_DIR)/rwx.o
	$(LD) -N $(ASSETS_DIR)/rwx.o -o $(ASSETS_DIR)/rwx
	rm -f $(ASSETS_DIR)/rwx.o

$(ASSETS_DIR)/hello_pie_partial_relro: hello.c
	$(CC) hello.c -fPIE -pie -Wl,-z,relro -o $(ASSETS_DIR)/hello_pie_partial_relro

$(ASSETS_DIR)/hello_nopie_full_relro: hello.c
	$(CC) hello.c -no-pie -Wl,-z,relro,-z,now -o $(ASSETS_DIR)/hello_nopie_full_relro

$(ASSETS_DIR)/hello_execstack: hello.c
	$(CC) hello.c -no-pie -Wl,-z,execstack -o $(ASSETS_DIR)/hello_execstack

$(ASSETS_DIR)/hello_norelro: hello.c
	$(CC) hello.c -no-pie -Wl,-z,norelro -o $(ASSETS_DIR)/hello_norelro

clean:
	rm -f $(ASSETS_DIR)/hello
	rm -f $(ASSETS_DIR)/hello.o
	rm -f $(ASSETS_DIR)/libhello.so
	rm -f $(ASSETS_DIR)/rwx
	rm -f $(ASSETS_DIR)/rwx.o
	rm -f $(ASSETS_DIR)/hello_pie_partial_relro
	rm -f $(ASSETS_DIR)/hello_nopie_full_relro
	rm -f $(ASSETS_DIR)/hello_execstack
	rm -f $(ASSETS_DIR)/hello_norelro