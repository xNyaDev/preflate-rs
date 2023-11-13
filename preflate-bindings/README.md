# preflate-bindings

Minimal Rust bindings to the [preflate](https://github.com/deus-libri/preflate) C++ library.

Provides wrappers only over 2 functions of the preflate library: `preflate_decode` and `preflate_reencode`, both in
their most basic form.

See the readme of `libpreflate-sys` for under-the-hood details.

## Example usage

```rs
// Prepare example data
let data = b"ATATATATATATATAT".to_vec();

let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
encoder.write_all(data.as_slice())?;
let compressed_data = encoder.finish()?;

// Compressed data will have a zlib header `78 9C`^1 and 4 bytes of checksum^2 `27 58 04 A9` at the end
// Preflate will not work with those so we need to discard them
//
// Footnotes:
// 1. zlib will usually start with the `78` byte and the next byte depends on the settings used. Exact 
//    specification of the 2 header bytes can be found at https://datatracker.ietf.org/doc/html/rfc1950
// 2. Adler-32 checksum of the decompressed data. The `adler`, `adler32` or `simd-adler32` crates can be used to 
//    calculate this if required.

let compressed_data = &compressed_data[2..(compressed_data.len() - 4)];

// preflate_decode returns Result<DecodeResult, PreflateError>
// DecodeResult has 2 fields:
// - unpacked_output which is the decompressed data
// - preflate_diff which is the reconstruction data
let decode_results = preflate_bindings::preflate_decode(compressed_data);

assert!(decode_results.is_ok());

let decode_results = decode_results.unwrap();

assert_eq!(data, decode_results.unpacked_output);

// preflate_reencode returns Result<Vec<u8>, PreflateError>
// The Vec<u8> is the recompressed data
let encode_results = preflate_bindings::preflate_reencode(&decode_results.preflate_diff, &data);

assert!(encode_results.is_ok());
assert_eq!(compressed_data, encode_results.unwrap());
```
