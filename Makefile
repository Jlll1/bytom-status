PREFIX=/usr/local/bin

build:
	rustup default stable
	cargo build --release

install: build
	chmod 755 target/release/bytom-status
	mkdir -p ${DESTDIR}${PREFIX}
	cp -f target/release/bytom-status ${DESTDIR}${PREFIX}/btms

uninstall:
	rm -f ${DESTDIR}${PREFIX}/btms

.PHONY: build install uninstall
