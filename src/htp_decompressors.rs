use crate::{htp_connection_parser, htp_transaction, htp_util, lzma};
use ::libc;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type htp_status_t = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_content_encoding_t {
    /**
     * This is the default value, which is used until the presence
     * of content encoding is determined (e.g., before request headers
     * are seen.
     */
    HTP_COMPRESSION_UNKNOWN,
    /** No compression. */
    HTP_COMPRESSION_NONE,
    /** Gzip compression. */
    HTP_COMPRESSION_GZIP,
    /** Deflate compression. */
    HTP_COMPRESSION_DEFLATE,
    /** LZMA compression. */
    HTP_COMPRESSION_LZMA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_decompressor_t {
    pub decompress: Option<
        unsafe extern "C" fn(
            _: *mut htp_decompressor_t,
            _: *mut htp_transaction::htp_tx_data_t,
        ) -> htp_status_t,
    >,
    pub callback:
        Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> htp_status_t>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut htp_decompressor_t) -> ()>,
    pub next: *mut htp_decompressor_t,
}

pub type htp_time_t = libc::timeval;

pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
/* 7zTypes.h -- Basic types
2018-08-04 : Igor Pavlov : Public domain */
pub type SRes = libc::c_int;
pub type UInt16 = libc::c_ushort;
pub type UInt32 = libc::c_uint;
pub type SizeT = size_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_decompressor_gzip_t {
    pub super_0: htp_decompressor_t,
    pub zlib_initialized: libc::c_int,
    pub restart: uint8_t,
    pub passthrough: uint8_t,
    pub stream: z_stream,
    pub header: [uint8_t; 13],
    pub header_len: uint8_t,
    pub state: lzma::LzmaDec::CLzmaDec,
    pub buffer: *mut libc::c_uchar,
    pub crc: libc::c_ulong,
}
unsafe extern "C" fn SzAlloc(
    mut _p: lzma::LzmaDec::ISzAllocPtr,
    mut size: size_t,
) -> *mut libc::c_void {
    return malloc(size);
}
unsafe extern "C" fn SzFree(mut _p: lzma::LzmaDec::ISzAllocPtr, mut address: *mut libc::c_void) {
    free(address);
}
#[no_mangle]
pub static mut lzma_Alloc: lzma::LzmaDec::ISzAlloc = {
    let mut init = lzma::LzmaDec::ISzAlloc {
        Alloc: Some(
            SzAlloc
                as unsafe extern "C" fn(
                    _: lzma::LzmaDec::ISzAllocPtr,
                    _: size_t,
                ) -> *mut libc::c_void,
        ),
        Free: Some(
            SzFree
                as unsafe extern "C" fn(_: lzma::LzmaDec::ISzAllocPtr, _: *mut libc::c_void) -> (),
        ),
    };
    init
};

/* *
 *  @brief See if the header has extensions
 *  @return number of bytes to skip
 */
unsafe extern "C" fn htp_gzip_decompressor_probe(
    mut data: *const libc::c_uchar,
    mut data_len: size_t,
) -> size_t {
    if data_len < 4 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    let mut consumed: size_t = 0 as libc::c_int as size_t;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x1f as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x8b as libc::c_int
        && *data.offset(3 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
    {
        if *data.offset(3 as libc::c_int as isize) as libc::c_int
            & (1 as libc::c_int) << 3 as libc::c_int
            != 0
            || *data.offset(3 as libc::c_int as isize) as libc::c_int
                & (1 as libc::c_int) << 4 as libc::c_int
                != 0
        {
            /* skip past
             * - FNAME extension, which is a name ended in a NUL terminator
             * or
             * - FCOMMENT extension, which is a commend ended in a NULL terminator
             */
            let mut len: size_t = 0;
            len = 10 as libc::c_int as size_t;
            while len < data_len && *data.offset(len as isize) as libc::c_int != '\u{0}' as i32 {
                len = len.wrapping_add(1)
            }
            consumed = len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else if *data.offset(3 as libc::c_int as isize) as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int
            != 0
        {
            consumed = 12 as libc::c_int as size_t
        //printf("skipped %u bytes for FHCRC header (GZIP)\n", 12);
        } else {
            //printf("GZIP unknown/unsupported flags %02X\n", data[3]);
            consumed = 10 as libc::c_int as size_t
        }
    }
    if consumed > data_len {
        return 0 as libc::c_int as size_t;
    }
    return consumed;
}

/* *
 *  @brief restart the decompressor
 *  @return 1 if it restarted, 0 otherwise
 */
unsafe extern "C" fn htp_gzip_decompressor_restart(
    mut drec: *mut htp_decompressor_gzip_t,
    mut data: *const libc::c_uchar,
    mut data_len: size_t,
    mut consumed_back: *mut size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut consumed: size_t = 0 as libc::c_int as size_t;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if ((*drec).restart as libc::c_int) < 3 as libc::c_int {
        // first retry with the existing type, but now consider the
        // extensions
        if (*drec).restart as libc::c_int == 0 as libc::c_int {
            consumed = htp_gzip_decompressor_probe(data, data_len);
            if (*drec).zlib_initialized
                == htp_content_encoding_t::HTP_COMPRESSION_GZIP as libc::c_int
            {
                // if that still fails, try the other method we support
                //printf("GZIP restart, consumed %u\n", (uint)consumed);
                rc = inflateInit2_(
                    &mut (*drec).stream,
                    15 as libc::c_int + 32 as libc::c_int,
                    b"1.2.11\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
                )
            } else {
                //printf("DEFLATE restart, consumed %u\n", (uint)consumed);
                rc = inflateInit2_(
                    &mut (*drec).stream,
                    -(15 as libc::c_int),
                    b"1.2.11\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
                )
            }
            if rc != 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            current_block = 5272667214186690925;
        } else if (*drec).zlib_initialized
            == htp_content_encoding_t::HTP_COMPRESSION_DEFLATE as libc::c_int
        {
            rc = inflateInit2_(
                &mut (*drec).stream,
                15 as libc::c_int + 32 as libc::c_int,
                b"1.2.11\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if rc != 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            (*drec).zlib_initialized = htp_content_encoding_t::HTP_COMPRESSION_GZIP as libc::c_int;
            consumed = htp_gzip_decompressor_probe(data, data_len);
            current_block = 5272667214186690925;
        } else if (*drec).zlib_initialized
            == htp_content_encoding_t::HTP_COMPRESSION_GZIP as libc::c_int
        {
            rc = inflateInit2_(
                &mut (*drec).stream,
                -(15 as libc::c_int),
                b"1.2.11\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if rc != 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            (*drec).zlib_initialized =
                htp_content_encoding_t::HTP_COMPRESSION_DEFLATE as libc::c_int;
            consumed = htp_gzip_decompressor_probe(data, data_len);
            current_block = 5272667214186690925;
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            14401909646449704462 => {}
            _ => {
                *consumed_back = consumed;
                (*drec).restart = (*drec).restart.wrapping_add(1);
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}

/* *
 * Ends decompressor.
 *
 * @param[in] drec
 */
unsafe extern "C" fn htp_gzip_decompressor_end(mut drec: *mut htp_decompressor_gzip_t) {
    if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_LZMA as libc::c_int {
        lzma::LzmaDec::LzmaDec_Free(&mut (*drec).state, &lzma_Alloc);
        (*drec).zlib_initialized = 0 as libc::c_int
    } else if (*drec).zlib_initialized != 0 {
        inflateEnd(&mut (*drec).stream);
        (*drec).zlib_initialized = 0 as libc::c_int
    };
}

/* *
 * Decompress a chunk of gzip-compressed data.
 * If we have more than one decompressor, call this function recursively.
 *
 * @param[in] drec
 * @param[in] d
 * @return HTP_OK on success, HTP_ERROR or some other negative integer on failure.
 */
unsafe extern "C" fn htp_gzip_decompressor_decompress(
    mut drec: *mut htp_decompressor_gzip_t,
    mut d: *mut htp_transaction::htp_tx_data_t,
) -> htp_status_t {
    let mut consumed: size_t = 0 as libc::c_int as size_t;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut callback_rc: htp_status_t = 0;
    // Pass-through the NULL chunk, which indicates the end of the stream.
    if (*drec).passthrough != 0 {
        let mut d2: htp_transaction::htp_tx_data_t = htp_transaction::htp_tx_data_t {
            tx: 0 as *mut htp_transaction::htp_tx_t,
            data: 0 as *const libc::c_uchar,
            len: 0,
            is_last: 0,
        };
        d2.tx = (*d).tx;
        d2.data = (*d).data;
        d2.len = (*d).len;
        d2.is_last = (*d).is_last;
        callback_rc = (*drec).super_0.callback.expect("non-null function pointer")(&mut d2);
        if callback_rc != 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        return 1 as libc::c_int;
    }
    if (*d).data.is_null() {
        // Prepare data for callback.
        let mut dout: htp_transaction::htp_tx_data_t = htp_transaction::htp_tx_data_t {
            tx: 0 as *mut htp_transaction::htp_tx_t,
            data: 0 as *const libc::c_uchar,
            len: 0,
            is_last: 0,
        };
        dout.tx = (*d).tx;
        // This is last call, so output uncompressed data so far
        dout.len =
            (8192 as libc::c_int as libc::c_uint).wrapping_sub((*drec).stream.avail_out) as size_t;
        if dout.len > 0 as libc::c_int as libc::c_ulong {
            dout.data = (*drec).buffer
        } else {
            dout.data = 0 as *const libc::c_uchar
        }
        dout.is_last = (*d).is_last;
        if !(*drec).super_0.next.is_null() && (*drec).zlib_initialized != 0 {
            return htp_gzip_decompressor_decompress(
                (*drec).super_0.next as *mut htp_decompressor_gzip_t,
                &mut dout,
            );
        } else {
            // Send decompressed data to the callback.
            callback_rc = (*drec).super_0.callback.expect("non-null function pointer")(&mut dout);
            if callback_rc != 1 as libc::c_int {
                htp_gzip_decompressor_end(drec);
                return callback_rc;
            }
        }
        return 1 as libc::c_int;
    }
    'c_5645: loop
    // we'll be restarting the compressor
    {
        if consumed > (*d).len {
            htp_util::htp_log(
                (*(*d).tx).connp,
                b"htp_decompressors.c\x00" as *const u8 as *const libc::c_char,
                235 as libc::c_int,
                htp_util::htp_log_level_t::HTP_LOG_ERROR,
                0 as libc::c_int,
                b"GZip decompressor: consumed > d->len\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*drec).stream.next_in = (*d).data.offset(consumed as isize) as *mut libc::c_uchar;
        (*drec).stream.avail_in = (*d).len.wrapping_sub(consumed) as uInt;
        while (*drec).stream.avail_in != 0 as libc::c_int as libc::c_uint {
            // If there's no more data left in the
            // buffer, send that information out.
            if (*drec).stream.avail_out == 0 as libc::c_int as libc::c_uint {
                (*drec).crc = crc32((*drec).crc, (*drec).buffer, 8192 as libc::c_int as uInt);
                // Prepare data for callback.
                let mut d2_0: htp_transaction::htp_tx_data_t = htp_transaction::htp_tx_data_t {
                    tx: 0 as *mut htp_transaction::htp_tx_t,
                    data: 0 as *const libc::c_uchar,
                    len: 0,
                    is_last: 0,
                };
                d2_0.tx = (*d).tx;
                d2_0.data = (*drec).buffer;
                d2_0.len = 8192 as libc::c_int as size_t;
                d2_0.is_last = (*d).is_last;
                if !(*drec).super_0.next.is_null() && (*drec).zlib_initialized != 0 {
                    callback_rc = htp_gzip_decompressor_decompress(
                        (*drec).super_0.next as *mut htp_decompressor_gzip_t,
                        &mut d2_0,
                    )
                } else {
                    // Send decompressed data to callback.
                    callback_rc =
                        (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_0)
                }
                if callback_rc != 1 as libc::c_int {
                    htp_gzip_decompressor_end(drec);
                    return callback_rc;
                }
                (*drec).stream.next_out = (*drec).buffer;
                (*drec).stream.avail_out = 8192 as libc::c_int as uInt
            }
            if (*drec).zlib_initialized
                == htp_content_encoding_t::HTP_COMPRESSION_LZMA as libc::c_int
            {
                if ((*drec).header_len as libc::c_int) < 5 as libc::c_int + 8 as libc::c_int {
                    consumed = (5 as libc::c_int + 8 as libc::c_int
                        - (*drec).header_len as libc::c_int)
                        as size_t;
                    if consumed > (*drec).stream.avail_in as libc::c_ulong {
                        consumed = (*drec).stream.avail_in as size_t
                    }
                    memcpy(
                        (*drec)
                            .header
                            .as_mut_ptr()
                            .offset((*drec).header_len as libc::c_int as isize)
                            as *mut libc::c_void,
                        (*drec).stream.next_in as *const libc::c_void,
                        consumed,
                    );
                    (*drec).stream.next_in =
                        (*d).data.offset(consumed as isize) as *mut libc::c_uchar;
                    (*drec).stream.avail_in = (*d).len.wrapping_sub(consumed) as uInt;
                    (*drec).header_len = ((*drec).header_len as libc::c_ulong)
                        .wrapping_add(consumed) as uint8_t
                        as uint8_t
                }
                if (*drec).header_len as libc::c_int == 5 as libc::c_int + 8 as libc::c_int {
                    rc = lzma::LzmaDec::LzmaDec_Allocate(
                        &mut (*drec).state,
                        (*drec).header.as_mut_ptr(),
                        5 as libc::c_int as libc::c_uint,
                        &lzma_Alloc,
                    );
                    if rc != 0 as libc::c_int {
                        return rc;
                    }
                    lzma::LzmaDec::LzmaDec_Init(&mut (*drec).state);
                    // hacky to get to next step end retry allocate in case of failure
                    (*drec).header_len = (*drec).header_len.wrapping_add(1)
                }
                if (*drec).header_len as libc::c_int > 5 as libc::c_int + 8 as libc::c_int {
                    let mut inprocessed: size_t = (*drec).stream.avail_in as size_t;
                    let mut outprocessed: size_t = (*drec).stream.avail_out as size_t;
                    let mut status = lzma::LzmaDec::ELzmaStatus::LZMA_STATUS_NOT_SPECIFIED;
                    rc = lzma::LzmaDec::LzmaDec_DecodeToBuf(
                        &mut (*drec).state,
                        (*drec).stream.next_out,
                        &mut outprocessed,
                        (*drec).stream.next_in,
                        &mut inprocessed,
                        lzma::LzmaDec::ELzmaFinishMode::LZMA_FINISH_ANY,
                        &mut status,
                        (*(*(*d).tx).cfg).lzma_memlimit,
                    );
                    (*drec).stream.avail_in = ((*drec).stream.avail_in as libc::c_ulong)
                        .wrapping_sub(inprocessed)
                        as uInt as uInt;
                    (*drec).stream.next_in = (*drec).stream.next_in.offset(inprocessed as isize);
                    (*drec).stream.avail_out = ((*drec).stream.avail_out as libc::c_ulong)
                        .wrapping_sub(outprocessed)
                        as uInt as uInt;
                    (*drec).stream.next_out = (*drec).stream.next_out.offset(outprocessed as isize);
                    let mut current_block_82: u64;
                    match rc {
                        0 => {
                            rc = 0 as libc::c_int;
                            if status as libc::c_uint
                                == lzma::LzmaDec::ELzmaStatus::LZMA_STATUS_FINISHED_WITH_MARK
                                    as libc::c_int
                                    as libc::c_uint
                            {
                                rc = 1 as libc::c_int
                            }
                            current_block_82 = 17019156190352891614;
                        }
                        2 => {
                            htp_util::htp_log(
                                (*(*d).tx).connp,
                                b"htp_decompressors.c\x00" as *const u8 as *const libc::c_char,
                                306 as libc::c_int,
                                htp_util::htp_log_level_t::HTP_LOG_WARNING,
                                0 as libc::c_int,
                                b"LZMA decompressor: memory limit reached\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block_82 = 1497605668091507245;
                        }
                        _ => {
                            current_block_82 = 1497605668091507245;
                        }
                    }
                    match current_block_82 {
                        1497605668091507245 =>
                        // fall through
                        {
                            rc = -(3 as libc::c_int)
                        }
                        _ => {}
                    }
                }
            } else if (*drec).zlib_initialized != 0 {
                rc = inflate(&mut (*drec).stream, 0 as libc::c_int)
            } else {
                // no initialization means previous error on stream
                return -(1 as libc::c_int);
            }
            if 8192 as libc::c_int as libc::c_uint > (*drec).stream.avail_out {
                if rc == -(3 as libc::c_int) {
                    // There is data even if there is an error
                    // So use this data and log a warning
                    htp_util::htp_log(
                        (*(*d).tx).connp,
                        b"htp_decompressors.c\x00" as *const u8 as *const libc::c_char,
                        322 as libc::c_int,
                        htp_util::htp_log_level_t::HTP_LOG_WARNING,
                        0 as libc::c_int,
                        b"GZip decompressor: inflate failed with %d\x00" as *const u8
                            as *const libc::c_char,
                        rc,
                    );
                    rc = 1 as libc::c_int
                }
            }
            if rc == 1 as libc::c_int {
                // How many bytes do we have?
                let mut len: size_t = (8192 as libc::c_int as libc::c_uint)
                    .wrapping_sub((*drec).stream.avail_out)
                    as size_t;
                // Update CRC
                // Prepare data for the callback.
                let mut d2_1: htp_transaction::htp_tx_data_t = htp_transaction::htp_tx_data_t {
                    tx: 0 as *mut htp_transaction::htp_tx_t,
                    data: 0 as *const libc::c_uchar,
                    len: 0,
                    is_last: 0,
                };
                d2_1.tx = (*d).tx;
                d2_1.data = (*drec).buffer;
                d2_1.len = len;
                d2_1.is_last = (*d).is_last;
                if !(*drec).super_0.next.is_null() && (*drec).zlib_initialized != 0 {
                    callback_rc = htp_gzip_decompressor_decompress(
                        (*drec).super_0.next as *mut htp_decompressor_gzip_t,
                        &mut d2_1,
                    )
                } else {
                    // Send decompressed data to the callback.
                    callback_rc =
                        (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_1)
                }
                if callback_rc != 1 as libc::c_int {
                    htp_gzip_decompressor_end(drec);
                    return callback_rc;
                }
                (*drec).stream.avail_out = 8192 as libc::c_int as uInt;
                (*drec).stream.next_out = (*drec).buffer;
                // TODO Handle trailer.
                return 1 as libc::c_int;
            } else {
                if !(rc != 0 as libc::c_int) {
                    continue;
                }
                htp_util::htp_log(
                    (*(*d).tx).connp,
                    b"htp_decompressors.c\x00" as *const u8 as *const libc::c_char,
                    356 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"GZip decompressor: inflate failed with %d\x00" as *const u8
                        as *const libc::c_char,
                    rc,
                );
                if (*drec).zlib_initialized
                    == htp_content_encoding_t::HTP_COMPRESSION_LZMA as libc::c_int
                {
                    lzma::LzmaDec::LzmaDec_Free(&mut (*drec).state, &lzma_Alloc);
                    // so as to clean zlib ressources after restart
                    (*drec).zlib_initialized =
                        htp_content_encoding_t::HTP_COMPRESSION_NONE as libc::c_int
                } else {
                    inflateEnd(&mut (*drec).stream);
                }
                // see if we want to restart the decompressor
                if htp_gzip_decompressor_restart(drec, (*d).data, (*d).len, &mut consumed)
                    == 1 as libc::c_int
                {
                    continue 'c_5645;
                }
                (*drec).zlib_initialized = 0 as libc::c_int;
                // all our inflate attempts have failed, simply
                // pass the raw data on to the callback in case
                // it's not compressed at all
                let mut d2_2: htp_transaction::htp_tx_data_t = htp_transaction::htp_tx_data_t {
                    tx: 0 as *mut htp_transaction::htp_tx_t,
                    data: 0 as *const libc::c_uchar,
                    len: 0,
                    is_last: 0,
                };
                d2_2.tx = (*d).tx;
                d2_2.data = (*d).data;
                d2_2.len = (*d).len;
                d2_2.is_last = (*d).is_last;
                callback_rc =
                    (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_2);
                if callback_rc != 1 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                (*drec).stream.avail_out = 8192 as libc::c_int as uInt;
                (*drec).stream.next_out = (*drec).buffer;
                /* successfully passed through, lets continue doing that */
                (*drec).passthrough = 1 as libc::c_int as uint8_t;
                return 1 as libc::c_int;
            }
        }
        return 1 as libc::c_int;
    }
}

/* *
 * Shut down gzip decompressor.
 *
 * @param[in] drec
 */
unsafe extern "C" fn htp_gzip_decompressor_destroy(mut drec: *mut htp_decompressor_gzip_t) {
    if drec.is_null() {
        return;
    }
    htp_gzip_decompressor_end(drec);
    free((*drec).buffer as *mut libc::c_void);
    free(drec as *mut libc::c_void);
}
/* *< deflate restarted to try rfc1950 instead of 1951 */
/* *< decompression failed, pass through raw data */
/* *
 * Create a new decompressor instance.
 *
 * @param[in] connp
 * @param[in] format
 * @return New htp_decompressor_t instance on success, or NULL on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_gzip_decompressor_create(
    mut connp: *mut htp_connection_parser::htp_connp_t,
    mut format: htp_content_encoding_t,
) -> *mut htp_decompressor_t {
    let mut drec: *mut htp_decompressor_gzip_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_decompressor_gzip_t>() as libc::c_ulong,
    ) as *mut htp_decompressor_gzip_t;
    if drec.is_null() {
        return 0 as *mut htp_decompressor_t;
    }
    (*drec).super_0.decompress = ::std::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                _: *mut htp_decompressor_gzip_t,
                _: *mut htp_transaction::htp_tx_data_t,
            ) -> htp_status_t,
        >,
        Option<
            unsafe extern "C" fn(
                _: *mut htp_decompressor_t,
                _: *mut htp_transaction::htp_tx_data_t,
            ) -> libc::c_int,
        >,
    >(Some(
        htp_gzip_decompressor_decompress
            as unsafe extern "C" fn(
                _: *mut htp_decompressor_gzip_t,
                _: *mut htp_transaction::htp_tx_data_t,
            ) -> htp_status_t,
    ));
    (*drec).super_0.destroy = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: *mut htp_decompressor_gzip_t) -> ()>,
        Option<unsafe extern "C" fn(_: *mut htp_decompressor_t) -> ()>,
    >(Some(
        htp_gzip_decompressor_destroy
            as unsafe extern "C" fn(_: *mut htp_decompressor_gzip_t) -> (),
    ));
    (*drec).super_0.next = 0 as *mut htp_decompressor_t;
    (*drec).buffer = malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    if (*drec).buffer.is_null() {
        free(drec as *mut libc::c_void);
        return 0 as *mut htp_decompressor_t;
    }
    // Initialize zlib.
    let mut rc: libc::c_int = 0;
    match format as libc::c_uint {
        4 => {
            if (*(*connp).cfg).lzma_memlimit > 0 as libc::c_int as libc::c_ulong {
                (*drec).state.dic = 0 as *mut Byte;
                (*drec).state.probs = 0 as *mut lzma::LzmaDec::CLzmaProb
            } else {
                htp_util::htp_log(
                    connp,
                    b"htp_decompressors.c\x00" as *const u8 as *const libc::c_char,
                    445 as libc::c_int,
                    htp_util::htp_log_level_t::HTP_LOG_WARNING,
                    0 as libc::c_int,
                    b"LZMA decompression disabled\x00" as *const u8 as *const libc::c_char,
                );
                (*drec).passthrough = 1 as libc::c_int as uint8_t
            }
            rc = 0 as libc::c_int
        }
        3 => {
            // Negative values activate raw processing,
            // which is what we need for deflate.
            rc = inflateInit2_(
                &mut (*drec).stream,
                -(15 as libc::c_int),
                b"1.2.11\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            )
        }
        2 => {
            // Increased windows size activates gzip header processing.
            rc = inflateInit2_(
                &mut (*drec).stream,
                15 as libc::c_int + 32 as libc::c_int,
                b"1.2.11\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            )
        }
        _ => {
            // do nothing
            rc = -(3 as libc::c_int)
        }
    }
    if rc != 0 as libc::c_int {
        htp_util::htp_log(
            connp,
            b"htp_decompressors.c\x00" as *const u8 as *const libc::c_char,
            465 as libc::c_int,
            htp_util::htp_log_level_t::HTP_LOG_ERROR,
            0 as libc::c_int,
            b"GZip decompressor: inflateInit2 failed with code %d\x00" as *const u8
                as *const libc::c_char,
            rc,
        );
        if format == htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
            || format == htp_content_encoding_t::HTP_COMPRESSION_GZIP
        {
            inflateEnd(&mut (*drec).stream);
        }
        free((*drec).buffer as *mut libc::c_void);
        free(drec as *mut libc::c_void);
        return 0 as *mut htp_decompressor_t;
    }
    (*drec).zlib_initialized = format as libc::c_int;
    (*drec).stream.avail_out = 8192 as libc::c_int as uInt;
    (*drec).stream.next_out = (*drec).buffer;
    return drec as *mut htp_decompressor_t;
}
