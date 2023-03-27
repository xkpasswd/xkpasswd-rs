.PHONY: all clean build install

all: clean build install

build:
	@makepkg --cleanbuild --syncdeps --needed --clean

install:
	@makepkg --install

clean:
	@rm -rf *.tar.zst *.tar.gz xkpasswd-rs-*
