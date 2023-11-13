#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("libpreflate-sys/vendor/preflate/preflate.h");
        fn preflate_decode(
            unpacked_output: Pin<&mut CxxVector<u8>>,
            preflate_diff: Pin<&mut CxxVector<u8>>,
            deflate_raw: &CxxVector<u8>,
            metaBlockSize: usize,
        ) -> bool;
        fn preflate_reencode(
            deflate_raw: Pin<&mut CxxVector<u8>>,
            preflate_diff: &CxxVector<u8>,
            unpacked_input: &CxxVector<u8>,
        ) -> bool;
    }
}

pub use ffi::*;
