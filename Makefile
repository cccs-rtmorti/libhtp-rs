
CARGO ?= cargo
DESTDIR ?= 
PREFIX ?= /usr
LIBDIR ?= $(PREFIX)/lib64
INCLUDEDIR ?= $(PREFIX)/include

# Use cargo to get the version or fallback to sed
$(eval CRATE_VERSION=$(shell \
	( \
		(${CARGO} 1> /dev/null 2> /dev/null) \
		&& (test -f Cargo.lock ||${CARGO} generate-lockfile) \
		&& (${CARGO} pkgid | cut -d\# -f 2 | cut -d@ -f 2 | cut -d: -f 2) \
	) \
	|| (sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml) \
))
$(eval CRATE_VERSION_MAJOR=$(shell echo ${CRATE_VERSION} | cut -d. -f 1))

# Source pattern to detect file changes and cache the build.
# Any source file change in the workspace should trigger a rebuild.
SOURCES := $(shell find . -path ./target -prune -false -o -type f \( -name "*.rs" -or -name "cbindgen.toml" -or -name "Cargo.toml" \) ) \
	Makefile
	
.PHONY: env
env:
	@echo CARGO: ${CARGO}
	@echo CRATE_VERSION: ${CRATE_VERSION}
	@echo CRATE_VERSION_MAJOR: ${CRATE_VERSION_MAJOR}
	@echo SOURCES: ${SOURCES}
	@echo DESTDIR: ${DESTDIR}
	@echo LIBDIR: $(LIBDIR)
	@echo INCLUDEDIR: ${INCLUDEDIR}

.PHONY: version
version:
	@echo ${CRATE_VERSION}

.DEFAULT_GOAL := all
.PHONY: all
all: headers shared_objects

.PHONY: clean
clean:
	${CARGO} clean

# Headers
# =======
.PHONY: headers
headers: htp/htp.h

htp/htp.h: ${SOURCES}
	RUSTUP_TOOLCHAIN=nightly cbindgen \
		--config cbindgen.toml \
		--crate htp \
		--output htp/htp.h \
		-v \
		--clean

# Shared Objects
# ==============
.PHONY: shared_objects
shared_objects: debug_objects release_objects

.PHONY: debug_objects
debug_objects: target/debug/libhtp.so 

.PHONY: release_objects
release_objects: target/release/libhtp.so 

target/debug/libhtp.so: ${SOURCES}
	${CARGO} build --features cbindgen

target/release/libhtp.so: ${SOURCES}
	RUSTFLAGS="-C link-arg=-Wl,-soname,$(@F).${CRATE_VERSION_MAJOR}" ${CARGO} build --features cbindgen --release

# prevents make check from failing in suricata
.PHONY: check
check:

# rpm
# ===
.PHONY: rpm
rpm: package
	rpmbuild -vvv -bb \
		--define "version ${CRATE_VERSION}" \
		--define "_topdir ${PWD}/target/rpmbuild" \
		--define "_prefix $(PREFIX)" \
		.rpm/htp.spec

.PHONY: package
package:
	rm -rf target/rpmbuild target/_temp
	mkdir -p target/rpmbuild/SOURCES target/_temp
	cp ${SOURCES} --parents target/_temp
	tar -czvf target/rpmbuild/SOURCES/libhtp-${CRATE_VERSION}.tar.gz target/_temp --transform 'flags=r;s#^target/_temp#libhtp-${CRATE_VERSION}#'

# note: symlinks must be relative to work with rpmbuild
.PHONY: install
install:
	install -d $(DESTDIR)$(LIBDIR)
	install -d $(DESTDIR)$(INCLUDEDIR)/htp
	install -m 0755 target/release/libhtp.so $(DESTDIR)$(LIBDIR)/libhtp.so.${CRATE_VERSION}
	cd $(DESTDIR)$(LIBDIR) && ln -s ./libhtp.so.${CRATE_VERSION_MAJOR} ./libhtp.so
	install -m 644 htp/*.h $(DESTDIR)$(INCLUDEDIR)/htp

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(LIBDIR)/libhtp*.so*
	rm -rf $(DESTDIR)$(INCLUDEDIR)/htp

.PHONY: valgrind 
valgrind:
	${CARGO} valgrind test --workspace --all-targets --all-features

.PHONY: asan-address
asan-address: export RUSTFLAGS = -Zsanitizer=address
asan-address: export RUSTDOCFLAGS = -Zsanitizer=address
asan-address:
	${CARGO} +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --workspace --all-targets --all-features

.PHONY: asan-memory
asan-memory: export RUSTFLAGS = -Zsanitizer=memory -Zsanitizer-memory-track-origins
asan-memory: export RUSTDOCFLAGS = -Zsanitizer=memory -Zsanitizer-memory-track-origins
asan-memory:
	${CARGO} +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --workspace --all-targets --all-features

.PHONY: asan-leak
asan-leak: export RUSTFLAGS = -Zsanitizer=leak
asan-leak: export RUSTDOCFLAGS = -Zsanitizer=leak
asan-leak:
	${CARGO} +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --workspace --all-targets --all-features

.PHONY: asan
asan: asan-address asan-memory asan-leak

# asan-address currently fails with `SIGILL` on functions with `extern "C"`
# so it is not included in memcheck until a solution is found
.PHONY: memcheck
memcheck: valgrind
