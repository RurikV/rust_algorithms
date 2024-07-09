use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

fn create_test_files() -> std::io::Result<()> {
    // Create a text file
    let mut text_file = File::create("text.txt")?;
    text_file.write_all(b"This is a sample text file. It contains repeating characters like aaaaabbbbbcccccddddd.")?;

    // Create a "photo" file (we'll use some pseudo-binary data)
    let mut photo_file = File::create("photo.jpg")?;
    for _ in 0..1000 {
        photo_file.write_all(&[0xFF, 0xD8, 0xFF, 0xE0])?;
    }

    // Create an "audio" file (we'll use some pseudo-binary data)
    let mut audio_file = File::create("audio.mp3")?;
    for _ in 0..1000 {
        audio_file.write_all(&[0x49, 0x44, 0x33, 0x03])?;
    }

    // Create a "zip" file (we'll use some pseudo-binary data)
    let mut zip_file = File::create("archive.zip")?;
    for _ in 0..1000 {
        zip_file.write_all(&[0x50, 0x4B, 0x03, 0x04])?;
    }

    Ok(())
}

fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    let compressed = rle_compress(&buffer);
    let mut output = File::create(output_path)?;

    for (count, byte) in compressed {
        output.write_all(&[count, byte])?;
    }

    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    let mut compressed = Vec::new();
    for chunk in buffer.chunks_exact(2) {
        compressed.push((chunk[0], chunk[1]));
    }

    let decompressed = rle_decompress(&compressed);
    let mut output = File::create(output_path)?;
    output.write_all(&decompressed)?;

    Ok(())
}

fn improved_compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    let compressed = improved_rle_compress(&buffer);
    let mut output = File::create(output_path)?;
    output.write_all(&compressed)?;

    Ok(())
}

fn improved_decompress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;

    let decompressed = improved_rle_decompress(&buffer);
    let mut output = File::create(output_path)?;
    output.write_all(&decompressed)?;

    Ok(())
}

fn rle_compress(data: &[u8]) -> Vec<(u8, u8)> {
    let mut compressed = Vec::new();
    let mut count = 1;
    let mut current = data[0];

    for &byte in data.iter().skip(1) {
        if byte == current && count < 255 {
            count += 1;
        } else {
            compressed.push((count, current));
            current = byte;
            count = 1;
        }
    }
    compressed.push((count, current));
    compressed
}

fn rle_decompress(compressed: &[(u8, u8)]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    for &(count, byte) in compressed {
        decompressed.extend(std::iter::repeat(byte).take(count as usize));
    }
    decompressed
}

fn improved_rle_compress(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut count = 1;
    let mut current = data[0];

    for &byte in data.iter().skip(1) {
        if byte == current && count < 65535 {
            count += 1;
        } else {
            if count < 128 {
                compressed.push(count as u8);
            } else {
                compressed.push(128 | (count >> 8) as u8);
                compressed.push((count & 0xFF) as u8);
            }
            compressed.push(current);
            current = byte;
            count = 1;
        }
    }

    if count < 128 {
        compressed.push(count as u8);
    } else {
        compressed.push(128 | (count >> 8) as u8);
        compressed.push((count & 0xFF) as u8);
    }
    compressed.push(current);

    compressed
}

fn improved_rle_decompress(compressed: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut i = 0;

    while i < compressed.len() {
        let mut count = compressed[i] as usize;
        i += 1;

        if count & 128 != 0 {
            count = ((count & 127) as usize) << 8 | compressed[i] as usize;
            i += 1;
        }

        let byte = compressed[i];
        i += 1;

        decompressed.extend(std::iter::repeat(byte).take(count));
    }

    decompressed
}


fn main() -> std::io::Result<()> {
    create_test_files()?;

    let file_types = ["text.txt", "photo.jpg", "audio.mp3", "archive.zip"];

    for file_type in &file_types {
        println!("Processing file: {}", file_type);

        // Original RLE
        let start = Instant::now();
        compress_file(file_type, &format!("{}.rle", file_type))?;
        let compress_time = start.elapsed();

        let start = Instant::now();
        decompress_file(&format!("{}.rle", file_type), &format!("{}.decompressed", file_type))?;
        let decompress_time = start.elapsed();

        // Improved RLE
        let start = Instant::now();
        improved_compress_file(file_type, &format!("{}.irle", file_type))?;
        let improved_compress_time = start.elapsed();

        let start = Instant::now();
        improved_decompress_file(&format!("{}.irle", file_type), &format!("{}.improved_decompressed", file_type))?;
        let improved_decompress_time = start.elapsed();

        // Calculate compression ratios
        let original_size = std::fs::metadata(file_type)?.len();
        let rle_size = std::fs::metadata(&format!("{}.rle", file_type))?.len();
        let irle_size = std::fs::metadata(&format!("{}.irle", file_type))?.len();

        println!("Original RLE - Compression time: {:?}, Decompression time: {:?}", compress_time, decompress_time);
        println!("Improved RLE - Compression time: {:?}, Decompression time: {:?}", improved_compress_time, improved_decompress_time);
        println!("Original size: {} bytes", original_size);
        println!("RLE compressed size: {} bytes (ratio: {:.2})", rle_size, rle_size as f64 / original_size as f64);
        println!("Improved RLE compressed size: {} bytes (ratio: {:.2})", irle_size, irle_size as f64 / original_size as f64);
        println!();
    }

    Ok(())
}
 