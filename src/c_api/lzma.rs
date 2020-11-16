use lzma_rs::decompress::{Options, Stream};
use std::io::{Cursor, Write};

#[repr(C)]
pub enum LZMA_STATUS {
    LZMA_STATUS_OK,
    LZMA_STATUS_ERROR,
}

#[no_mangle]
pub static LZMA_PROPS_SIZE: usize = 5;

/// Use the lzma algorithm to decompress a chunk of data.
///
/// Returns Status::OK on success, Status::ERROR otherwise.
#[no_mangle]
pub extern "C" fn lzma_decompress(
    input: *const u8,
    input_len: &mut usize,
    output: *mut u8,
    output_len: &mut usize,
    allow_incomplete: bool,
    memlimit: usize,
) -> LZMA_STATUS {
    let input = unsafe { std::slice::from_raw_parts(input, *input_len) };
    let output = unsafe { std::slice::from_raw_parts_mut(output, *output_len) };
    let output = Cursor::new(output);

    let options = Options {
        memlimit: Some(memlimit),
        allow_incomplete,
        ..Default::default()
    };

    let mut stream = Stream::new_with_options(&options, output);

    if let Err(_) = stream.write_all(&input[..]) {
        if !allow_incomplete {
            return LZMA_STATUS::LZMA_STATUS_ERROR;
        }
    }

    if let Ok(output) = stream.finish() {
        *output_len = output.position() as usize;
        LZMA_STATUS::LZMA_STATUS_OK
    } else {
        LZMA_STATUS::LZMA_STATUS_ERROR
    }
}
