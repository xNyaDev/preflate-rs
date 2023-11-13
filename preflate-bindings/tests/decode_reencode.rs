use std::error::Error;
use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use pretty_assertions::assert_eq;

#[test]
fn decode_reencode() -> Result<(), Box<dyn Error>> {
    let data = b"ATATATATATATATAT".to_vec();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_slice())?;
    let compressed_data = encoder.finish()?;

    let compressed_data = &compressed_data[2..(compressed_data.len() - 4)];

    let decode_results = preflate_bindings::preflate_decode(compressed_data);

    assert!(decode_results.is_ok());

    let decode_results = decode_results.unwrap();

    assert_eq!(data, decode_results.unpacked_output);

    let encode_results = preflate_bindings::preflate_reencode(&decode_results.preflate_diff, &data);

    assert!(encode_results.is_ok());
    assert_eq!(compressed_data, encode_results.unwrap());

    Ok(())
}
