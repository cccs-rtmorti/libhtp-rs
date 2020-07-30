use crate::{htp_connection_parser, htp_transaction, lzma, Status};
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn malloc(_: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
    #[no_mangle]
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: libc::size_t,
    ) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: libc::c_ulong, buf: *const libc::c_uchar, len: libc::c_uint) -> libc::c_ulong;
    #[no_mangle]
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_content_encoding_t {
    /// This is the default value, which is used until the presence
    /// of content encoding is determined (e.g., before request headers
    /// are seen.
    HTP_COMPRESSION_UNKNOWN,
    /// No compression.
    HTP_COMPRESSION_NONE,
    /// Gzip compression.
    HTP_COMPRESSION_GZIP,
    /// Deflate compression.
    HTP_COMPRESSION_DEFLATE,
    /// LZMA compression.
    HTP_COMPRESSION_LZMA,
    /// Error retrieving the content encoding.
    HTP_COMPRESSION_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_decompressor_t {
    pub decompress: Option<
        unsafe extern "C" fn(
            _: *mut htp_decompressor_t,
            _: *mut htp_transaction::htp_tx_data_t,
        ) -> Status,
    >,
    pub callback: Option<unsafe extern "C" fn(_: *mut htp_transaction::htp_tx_data_t) -> Status>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut htp_decompressor_t) -> ()>,
    pub next: *mut htp_decompressor_t,
    pub time_before: libc::timeval,
    pub time_spent: i32,
    pub nb_callbacks: u32,
}

pub type alloc_func = Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u32, _: u32) -> *mut core::ffi::c_void,
>;
pub type free_func =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct z_stream_s {
    pub next_in: *mut u8,
    pub avail_in: u32,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: u32,
    pub total_out: u64,
    pub msg: *mut u8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: *mut core::ffi::c_void,
    pub data_type: i32,
    pub adler: u64,
    pub reserved: u64,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_decompressor_gzip_t {
    pub super_0: htp_decompressor_t,
    pub zlib_initialized: i32,
    pub restart: u8,
    pub passthrough: u8,
    pub stream: z_stream,
    pub header: [u8; 13],
    pub header_len: u8,
    pub state: lzma::LzmaDec::CLzmaDec,
    pub buffer: *mut u8,
    pub crc: u64,
}
unsafe fn SzAlloc(mut _p: lzma::LzmaDec::ISzAllocPtr, size: usize) -> *mut core::ffi::c_void {
    malloc(size)
}
unsafe fn SzFree(mut _p: lzma::LzmaDec::ISzAllocPtr, address: *mut core::ffi::c_void) {
    free(address);
}
#[no_mangle]
pub static mut lzma_Alloc: lzma::LzmaDec::ISzAlloc = {
    lzma::LzmaDec::ISzAlloc {
        Alloc: Some(
            SzAlloc as unsafe fn(_: lzma::LzmaDec::ISzAllocPtr, _: usize) -> *mut core::ffi::c_void,
        ),
        Free: Some(
            SzFree as unsafe fn(_: lzma::LzmaDec::ISzAllocPtr, _: *mut core::ffi::c_void) -> (),
        ),
    }
};

///  See if the header has extensions
///
///  Returns number of bytes to skip
unsafe extern "C" fn htp_gzip_decompressor_probe(data: *const u8, data_len: usize) -> usize {
    if data_len < 4 {
        return 0;
    }
    let mut consumed: usize = 0;
    if *data.offset(0) == 0x1f && *data.offset(1) == 0x8b && *data.offset(3) != 0 {
        if *data.offset(3) & (1) << 3 != 0 || *data.offset(3) & (1) << 4 != 0 {
            // skip past
            // - FNAME extension, which is a name ended in a NUL terminator
            // or
            // - FCOMMENT extension, which is a commend ended in a NULL terminator
            let mut len: usize = 10;
            while len < data_len && *data.offset(len as isize) != 0 {
                len = len.wrapping_add(1)
            }
            consumed = len.wrapping_add(1)
        } else if *data.offset(3) & 1 << 1 != 0 {
            consumed = 12
        } else {
            consumed = 10
        }
    }
    if consumed > data_len {
        return 0;
    }
    consumed
}

///  restart the decompressor
///
///  Returns 1 if it restarted, 0 otherwise
unsafe extern "C" fn htp_gzip_decompressor_restart(
    mut drec: *mut htp_decompressor_gzip_t,
    data: *const u8,
    data_len: usize,
    consumed_back: *mut usize,
) -> i32 {
    let current_block: u64;
    let mut consumed: usize = 0;
    let mut rc: i32 = 0;
    if ((*drec).restart) < 3 {
        // first retry with the existing type, but now consider the
        // extensions
        if (*drec).restart == 0 {
            consumed = htp_gzip_decompressor_probe(data, data_len);
            if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_GZIP as i32 {
                // if that still fails, try the other method we support
                rc = inflateInit2_(
                    &mut (*drec).stream,
                    15 + 32,
                    b"1.2.11\x00" as *const u8 as *const i8,
                    ::std::mem::size_of::<z_stream>() as i32,
                )
            } else {
                rc = inflateInit2_(
                    &mut (*drec).stream,
                    -15,
                    b"1.2.11\x00" as *const u8 as *const i8,
                    ::std::mem::size_of::<z_stream>() as i32,
                )
            }
            if rc != 0 {
                return 0;
            }
            current_block = 5272667214186690925;
        } else if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_DEFLATE as i32
        {
            rc = inflateInit2_(
                &mut (*drec).stream,
                15 + 32,
                b"1.2.11\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<z_stream>() as i32,
            );
            if rc != 0 {
                return 0;
            }
            (*drec).zlib_initialized = htp_content_encoding_t::HTP_COMPRESSION_GZIP as i32;
            consumed = htp_gzip_decompressor_probe(data, data_len);
            current_block = 5272667214186690925;
        } else if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_GZIP as i32 {
            rc = inflateInit2_(
                &mut (*drec).stream,
                -15,
                b"1.2.11\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<z_stream>() as i32,
            );
            if rc != 0 {
                return 0;
            }
            (*drec).zlib_initialized = htp_content_encoding_t::HTP_COMPRESSION_DEFLATE as i32;
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
                return 1;
            }
        }
    }
    0
}

/// Ends decompressor.
unsafe fn htp_gzip_decompressor_end(mut drec: *mut htp_decompressor_gzip_t) {
    if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_LZMA as i32 {
        lzma::LzmaDec::LzmaDec_Free(&mut (*drec).state, &lzma_Alloc);
        (*drec).zlib_initialized = 0
    } else if (*drec).zlib_initialized != 0 {
        inflateEnd(&mut (*drec).stream);
        (*drec).zlib_initialized = 0
    };
}

/// Decompress a chunk of gzip-compressed data.
/// If we have more than one decompressor, call this function recursively.
///
/// Returns HTP_OK on success, HTP_ERROR or some other negative integer on failure.
unsafe extern "C" fn htp_gzip_decompressor_decompress(
    mut drec: *mut htp_decompressor_gzip_t,
    d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    let mut consumed: usize = 0;
    let mut rc: i32 = 0;
    let mut callback_rc: Status = Status::DECLINED;
    // Pass-through the NULL chunk, which indicates the end of the stream.
    if (*drec).passthrough != 0 {
        let mut d2 = (*d).clone();
        callback_rc = (*drec).super_0.callback.expect("non-null function pointer")(&mut d2);
        if callback_rc != Status::OK {
            return Status::ERROR;
        }
        return Status::OK;
    }
    if (*d).data().is_null() {
        // Prepare data for callback.
        let mut dout = {
            let len = (8192_usize).wrapping_sub((*drec).stream.avail_out as usize);
            let data = if len > 0 {
                (*drec).buffer
            } else {
                std::ptr::null()
            };
            htp_transaction::htp_tx_data_t::new((*d).tx(), data, len, (*d).is_last())
        };
        if !(*drec).super_0.next.is_null() && (*drec).zlib_initialized != 0 {
            return htp_gzip_decompressor_decompress(
                (*drec).super_0.next as *mut htp_decompressor_gzip_t,
                &mut dout,
            );
        } else {
            // Send decompressed data to the callback.
            callback_rc = (*drec).super_0.callback.expect("non-null function pointer")(&mut dout);
            if callback_rc != Status::OK {
                htp_gzip_decompressor_end(drec);
                return callback_rc;
            }
        }
        return Status::OK;
    }
    'c_5645: loop
    // we'll be restarting the compressor
    {
        let connp = (*(*d).tx()).connp;
        if consumed > (*d).len() {
            htp_error!(
                connp,
                htp_log_code::GZIP_DECOMPRESSION_FAILED,
                "GZip decompressor: consumed > d->len"
            );
            return Status::ERROR;
        }
        (*drec).stream.next_in = (*d).data().offset(consumed as isize) as *mut u8;
        (*drec).stream.avail_in = (*d).len().wrapping_sub(consumed) as u32;
        while (*drec).stream.avail_in != 0 {
            // If there's no more data left in the
            // buffer, send that information out.
            if (*drec).stream.avail_out == 0 {
                (*drec).crc = crc32((*drec).crc, (*drec).buffer, 8192);
                // Prepare data for callback.
                let mut d2_0 = htp_transaction::htp_tx_data_t::new(
                    (*d).tx(),
                    (*drec).buffer,
                    8192,
                    (*d).is_last(),
                );
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
                if callback_rc != Status::OK {
                    htp_gzip_decompressor_end(drec);
                    return callback_rc;
                }
                (*drec).stream.next_out = (*drec).buffer;
                (*drec).stream.avail_out = 8192
            }
            if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_LZMA as i32 {
                if ((*drec).header_len) < 5 + 8 {
                    consumed = (5 + 8 - (*drec).header_len) as usize;
                    if consumed > (*drec).stream.avail_in as usize {
                        consumed = (*drec).stream.avail_in as usize
                    }
                    memcpy(
                        (*drec)
                            .header
                            .as_mut_ptr()
                            .offset((*drec).header_len as isize)
                            as *mut core::ffi::c_void,
                        (*drec).stream.next_in as *const core::ffi::c_void,
                        consumed,
                    );
                    (*drec).stream.next_in = (*d).data().offset(consumed as isize) as *mut u8;
                    (*drec).stream.avail_in = (*d).len().wrapping_sub(consumed) as u32;
                    (*drec).header_len = ((*drec).header_len as usize).wrapping_add(consumed) as u8
                }
                if (*drec).header_len == 5 + 8 {
                    rc = lzma::LzmaDec::LzmaDec_Allocate(
                        &mut (*drec).state,
                        (*drec).header.as_mut_ptr(),
                        5,
                        &lzma_Alloc,
                    );
                    if rc != 0 {
                        match rc {
                            0 => return Status::OK,
                            _ => return Status::ERROR,
                        }
                    }
                    lzma::LzmaDec::LzmaDec_Init(&mut (*drec).state);
                    // hacky to get to next step end retry allocate in case of failure
                    (*drec).header_len = (*drec).header_len.wrapping_add(1)
                }
                if (*drec).header_len > 5 + 8 {
                    let mut inprocessed: usize = (*drec).stream.avail_in as usize;
                    let mut outprocessed: usize = (*drec).stream.avail_out as usize;
                    let mut status = lzma::LzmaDec::ELzmaStatus::LZMA_STATUS_NOT_SPECIFIED;
                    rc = lzma::LzmaDec::LzmaDec_DecodeToBuf(
                        &mut (*drec).state,
                        (*drec).stream.next_out,
                        &mut outprocessed as *mut usize,
                        (*drec).stream.next_in,
                        &mut inprocessed as *mut usize,
                        lzma::LzmaDec::ELzmaFinishMode::LZMA_FINISH_ANY,
                        &mut status,
                        (*(*(*d).tx()).cfg).lzma_memlimit,
                    );
                    (*drec).stream.avail_in =
                        ((*drec).stream.avail_in as usize).wrapping_sub(inprocessed) as u32;
                    (*drec).stream.next_in = (*drec).stream.next_in.offset(inprocessed as isize);
                    (*drec).stream.avail_out =
                        ((*drec).stream.avail_out as usize).wrapping_sub(outprocessed) as u32;
                    (*drec).stream.next_out = (*drec).stream.next_out.offset(outprocessed as isize);
                    let current_block_82: u64;
                    match rc {
                        0 => {
                            rc = 0;
                            if status == lzma::LzmaDec::ELzmaStatus::LZMA_STATUS_FINISHED_WITH_MARK
                            {
                                rc = 1
                            }
                            current_block_82 = 17019156190352891614;
                        }
                        2 => {
                            htp_warn!(
                                connp,
                                htp_log_code::LZMA_MEMLIMIT_REACHED,
                                "LZMA decompressor: memory limit reached"
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
                            rc = -3
                        }
                        _ => {}
                    }
                }
            } else if (*drec).zlib_initialized != 0 {
                rc = inflate(&mut (*drec).stream, 0)
            } else {
                // no initialization means previous error on stream
                return Status::ERROR;
            }
            if 8192 > (*drec).stream.avail_out && rc == -3 {
                // There is data even if there is an error
                // So use this data and log a warning
                htp_warn!(
                    connp,
                    htp_log_code::GZIP_DECOMPRESSION_FAILED,
                    format!("GZip decompressor: inflate failed with {}", rc)
                );
                rc = 1;
            }
            if rc == 1 {
                // How many bytes do we have?
                let len: usize = 8192_u32.wrapping_sub((*drec).stream.avail_out) as usize;
                // Update CRC
                // Prepare data for the callback.
                let mut d2_1 = htp_transaction::htp_tx_data_t::new(
                    (*d).tx(),
                    (*drec).buffer,
                    len,
                    (*d).is_last(),
                );
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
                if callback_rc != Status::OK {
                    htp_gzip_decompressor_end(drec);
                    return callback_rc;
                }
                (*drec).stream.avail_out = 8192;
                (*drec).stream.next_out = (*drec).buffer;
                // TODO Handle trailer.
                return Status::OK;
            } else {
                if !(rc != 0) {
                    continue;
                }
                htp_warn!(
                    connp,
                    htp_log_code::GZIP_DECOMPRESSION_FAILED,
                    format!("GZip decompressor: inflate failed with {}", rc)
                );
                if (*drec).zlib_initialized == htp_content_encoding_t::HTP_COMPRESSION_LZMA as i32 {
                    lzma::LzmaDec::LzmaDec_Free(&mut (*drec).state, &lzma_Alloc);
                    // so as to clean zlib ressources after restart
                    (*drec).zlib_initialized = htp_content_encoding_t::HTP_COMPRESSION_NONE as i32
                } else {
                    inflateEnd(&mut (*drec).stream);
                }
                // see if we want to restart the decompressor
                if htp_gzip_decompressor_restart(drec, (*d).data(), (*d).len(), &mut consumed) == 1
                {
                    continue 'c_5645;
                }
                (*drec).zlib_initialized = 0;
                // all our inflate attempts have failed, simply
                // pass the raw data on to the callback in case
                // it's not compressed at all
                let mut d2_2 = (*d).clone();
                callback_rc =
                    (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_2);
                if callback_rc != Status::OK {
                    return Status::ERROR;
                }
                (*drec).stream.avail_out = 8192;
                (*drec).stream.next_out = (*drec).buffer;
                // successfully passed through, lets continue doing that
                (*drec).passthrough = 1;
                return Status::OK;
            }
        }
        return Status::OK;
    }
}

/// Shut down gzip decompressor.
unsafe extern "C" fn htp_gzip_decompressor_destroy(drec: *mut htp_decompressor_gzip_t) {
    if drec.is_null() {
        return;
    }
    htp_gzip_decompressor_end(drec);
    free((*drec).buffer as *mut core::ffi::c_void);
    free(drec as *mut core::ffi::c_void);
}
// *< deflate restarted to try rfc1950 instead of 1951
// *< decompression failed, pass through raw data

/// Create a new decompressor instance.
///
/// Returns New htp_decompressor_t instance on success, or NULL on failure.
pub unsafe fn htp_gzip_decompressor_create(
    connp: *mut htp_connection_parser::htp_connp_t,
    format: htp_content_encoding_t,
) -> *mut htp_decompressor_t {
    let mut drec: *mut htp_decompressor_gzip_t =
        calloc(1, ::std::mem::size_of::<htp_decompressor_gzip_t>()) as *mut htp_decompressor_gzip_t;
    if drec.is_null() {
        return 0 as *mut htp_decompressor_t;
    }
    (*drec).super_0.decompress = ::std::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                _: *mut htp_decompressor_gzip_t,
                _: *mut htp_transaction::htp_tx_data_t,
            ) -> Status,
        >,
        Option<
            unsafe extern "C" fn(
                _: *mut htp_decompressor_t,
                _: *mut htp_transaction::htp_tx_data_t,
            ) -> Status,
        >,
    >(Some(
        htp_gzip_decompressor_decompress
            as unsafe extern "C" fn(
                _: *mut htp_decompressor_gzip_t,
                _: *mut htp_transaction::htp_tx_data_t,
            ) -> Status,
    ));
    (*drec).super_0.destroy = ::std::mem::transmute::<
        Option<unsafe extern "C" fn(_: *mut htp_decompressor_gzip_t) -> ()>,
        Option<unsafe extern "C" fn(_: *mut htp_decompressor_t) -> ()>,
    >(Some(
        htp_gzip_decompressor_destroy
            as unsafe extern "C" fn(_: *mut htp_decompressor_gzip_t) -> (),
    ));
    (*drec).super_0.next = 0 as *mut htp_decompressor_t;
    (*drec).buffer = malloc(8192) as *mut u8;
    if (*drec).buffer.is_null() {
        free(drec as *mut core::ffi::c_void);
        return 0 as *mut htp_decompressor_t;
    }
    // Initialize zlib.
    let mut rc: i32 = 0;
    match format {
        htp_content_encoding_t::HTP_COMPRESSION_LZMA => {
            if (*(*connp).cfg).lzma_memlimit > 0 {
                (*drec).state.dic = 0 as *mut u8;
                (*drec).state.probs = 0 as *mut lzma::LzmaDec::CLzmaProb
            } else {
                htp_warn!(
                    connp,
                    htp_log_code::LZMA_DECOMPRESSION_DISABLED,
                    "LZMA decompression disabled"
                );
                (*drec).passthrough = 1
            }
            rc = 0
        }
        htp_content_encoding_t::HTP_COMPRESSION_DEFLATE => {
            // Negative values activate raw processing,
            // which is what we need for deflate.
            rc = inflateInit2_(
                &mut (*drec).stream,
                -15,
                b"1.2.11\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<z_stream>() as i32,
            )
        }
        htp_content_encoding_t::HTP_COMPRESSION_GZIP => {
            // Increased windows size activates gzip header processing.
            rc = inflateInit2_(
                &mut (*drec).stream,
                15 + 32,
                b"1.2.11\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<z_stream>() as i32,
            )
        }
        _ => {
            // do nothing
            rc = -3
        }
    }

    if rc != 0 {
        htp_error!(
            connp,
            htp_log_code::GZIP_DECOMPRESSION_FAILED,
            format!("GZip decompressor: inflateInit2 failed with code {}", rc)
        );
        if format == htp_content_encoding_t::HTP_COMPRESSION_DEFLATE
            || format == htp_content_encoding_t::HTP_COMPRESSION_GZIP
        {
            inflateEnd(&mut (*drec).stream);
        }
        free((*drec).buffer as *mut core::ffi::c_void);
        free(drec as *mut core::ffi::c_void);
        return 0 as *mut htp_decompressor_t;
    }
    (*drec).zlib_initialized = format as i32;
    (*drec).stream.avail_out = 8192;
    (*drec).stream.next_out = (*drec).buffer;
    return drec as *mut htp_decompressor_t;
}
