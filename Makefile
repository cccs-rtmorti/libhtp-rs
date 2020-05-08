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
	cp ./target/debug/libhtp.so htp/.libs/libhtp.so.2.0.0
	ln -sf libhtp.so.2.0.0 htp/.libs/libhtp.so.2
	ln -sf libhtp.so.2.0.0 htp/.libs/libhtp.so
	

clean:
	cargo clean
