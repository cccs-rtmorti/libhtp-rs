$(eval CRATE_VERSION=$(shell cargo pkgid | cut -d: -f 3))
$(eval CRATE_VERSION_MAJOR=$(shell cargo pkgid | cut -d: -f 3 | cut -d. -f 1))

.PHONY: all
all:
	cargo build --features cbindgen
	mkdir -p htp/.libs
	mkdir -p htp/lzma
	mv htp.h htp/
	mv version.h htp/
	cp src/c_api/forward_decls.h htp/
	cp src/c_api/libhtp.la htp/
	cp src/c_api/7zTypes.h htp/lzma/
	cp src/c_api/LzmaDec.h htp/lzma/
	cp ./target/debug/libhtp.so htp/.libs/libhtp.so.${CRATE_VERSION}
	ln -sf libhtp.so.${CRATE_VERSION} htp/.libs/libhtp.so.${CRATE_VERSION_MAJOR}
	ln -sf libhtp.so.${CRATE_VERSION} htp/.libs/libhtp.so
	
.PHONY: clean
clean:
	cargo clean

.PHONY: rpm
rpm: tar
	rpmbuild -bb --define "version ${CRATE_VERSION}" --define "_topdir ${PWD}/target/centos" .rpm/htp.spec

.PHONY: tar
tar: all
	mkdir -p target/_temp/include/htp/lzma
	mkdir -p target/_temp/lib
	mkdir -p target/centos/
	mkdir -p target/centos/SOURCES
	cp htp/*.h target/_temp/include/htp
	cp htp/lzma/* target/_temp/include/htp/lzma
	cp -d htp/.libs/*.so* target/_temp/lib
	tar -czvf target/centos/SOURCES/libhtp-${CRATE_VERSION}.tar.gz target/_temp/ --transform 'flags=r;s#^target/_temp/#libhtp-${CRATE_VERSION}/usr/local/#'
