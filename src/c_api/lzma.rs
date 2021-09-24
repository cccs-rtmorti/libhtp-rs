use lzma_rs::decompress::{Options, Stream};
use std::io::{Cursor, Write};

/// LZMA_STATUS is a type that represents either success or failure.
#[repr(C)]
pub enum LZMA_STATUS {
    /// Status is successful
    LZMA_STATUS_OK,
    /// An error occurred
    LZMA_STATUS_ERROR,
}

#[no_mangle]
/// Size of encoded properties in header.
pub static LZMA_PROPS_SIZE: usize = 5;

/// Use the lzma algorithm to decompress a chunk of data.
///
/// Returns Status::OK on success, Status::ERROR otherwise.
/// # Safety
/// When calling this method, you have to ensure that input and output are both valid for reads of lengths input_len and output_len, respectively.
#[no_mangle]
pub unsafe extern "C" fn lzma_decompress(
    input: *const u8,
    input_len: &mut usize,
    output: *mut u8,
    output_len: &mut usize,
    allow_incomplete: bool,
    memlimit: usize,
) -> LZMA_STATUS {
    let input = std::slice::from_raw_parts(input, *input_len);
    let output = std::slice::from_raw_parts_mut(output, *output_len);
    let output = Cursor::new(output);

    let options = Options {
        memlimit: Some(memlimit),
        allow_incomplete,
        ..Default::default()
    };

    let mut stream = Stream::new_with_options(&options, output);

    if stream.write_all(input).is_err() && !allow_incomplete {
        return LZMA_STATUS::LZMA_STATUS_ERROR;
    }

    if let Ok(output) = stream.finish() {
        *output_len = output.position() as usize;
        LZMA_STATUS::LZMA_STATUS_OK
    } else {
        LZMA_STATUS::LZMA_STATUS_ERROR
    }
}
