use std::io::Write;

use brotli;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let quality: u32 = 9;
        let lg_window_size: u32 = 20;
        let mut writer = brotli::CompressorWriter::new(
            &mut output,
            4096, /* buffer size */
            quality,
            lg_window_size as u32,
        );
        writer.write(&data).unwrap();
    }
    output
}
