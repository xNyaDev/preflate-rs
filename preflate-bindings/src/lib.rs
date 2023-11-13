use cxx::CxxVector;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodeResult {
    pub unpacked_output: Vec<u8>,
    pub preflate_diff: Vec<u8>,
}

#[derive(Debug)]
pub struct PreflateError;

impl fmt::Display for PreflateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "preflate call failed")
    }
}

impl std::error::Error for PreflateError {}

pub fn preflate_decode(deflate_raw: &[u8]) -> Result<DecodeResult, PreflateError> {
    let mut unpacked_output_ffi = CxxVector::new();
    let mut preflate_diff_ffi = CxxVector::new();

    let mut deflate_raw_ffi = CxxVector::new();
    for byte in deflate_raw {
        deflate_raw_ffi.pin_mut().push(*byte);
    }

    let result = libpreflate_sys::preflate_decode(
        unpacked_output_ffi.pin_mut(),
        preflate_diff_ffi.pin_mut(),
        &deflate_raw_ffi,
        i32::MAX as usize,
    );

    let unpacked_output = unpacked_output_ffi.into_iter().cloned().collect();
    let preflate_diff = preflate_diff_ffi.into_iter().cloned().collect();

    if result {
        Ok(DecodeResult {
            unpacked_output,
            preflate_diff,
        })
    } else {
        Err(PreflateError)
    }
}

pub fn preflate_reencode(
    preflate_diff: &[u8],
    unpacked_input: &[u8],
) -> Result<Vec<u8>, PreflateError> {
    let mut unpacked_input_ffi = CxxVector::new();
    for byte in unpacked_input {
        unpacked_input_ffi.pin_mut().push(*byte);
    }
    let mut preflate_diff_ffi = CxxVector::new();
    for byte in preflate_diff {
        preflate_diff_ffi.pin_mut().push(*byte);
    }

    let mut deflate_raw_ffi = CxxVector::new();

    let result = libpreflate_sys::preflate_reencode(
        deflate_raw_ffi.pin_mut(),
        &preflate_diff_ffi,
        &unpacked_input_ffi,
    );

    let deflate_raw = deflate_raw_ffi.into_iter().cloned().collect();

    if result {
        Ok(deflate_raw)
    } else {
        Err(PreflateError)
    }
}
