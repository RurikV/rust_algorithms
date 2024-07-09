use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;

fn create_test_files() -> std::io::Result<()> {
    // Create a text file
    let mut text_file = File::create("text.txt")?;
    text_file.write_all(b"This is a sample text file. It contains repeating characters like aaaaabbbbbcccccddddd.")?;

    // Create a "photo" file 
    let mut photo_file = File::create("photo.jpg")?;
    photo_file.write_all(&[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01, 0x00, 0x60, 0x00, 0x60, 0x00, 0x00, 0xFF, 0xE1, 0x00, 0x5A, 0x45, 0x78, 0x69, 0x66, 0x00, 0x00, 0x4D, 0x4D, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x08, 0x00, 0x05, 0x03, 0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4A, 0x03, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x51, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x51, 0x11, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x0E, 0xC3, 0x51, 0x12, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x0E, 0xC3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x86, 0xA0, 0x00, 0x00, 0xB1, 0x8F, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x02, 0x01, 0x01, 0x02, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x03, 0x05, 0x03, 0x03, 0x03, 0x03, 0x03, 0x06, 0x04, 0x04, 0x03, 0x05, 0x07, 0x06, 0x07, 0x07, 0x07, 0x06, 0x07, 0x07, 0x08, 0x09, 0x0B, 0x09, 0x08, 0x08, 0x0A, 0x08, 0x07, 0x07, 0x0A, 0x0D, 0x0A, 0x0A, 0x0B, 0x0C, 0x0C, 0x0C, 0x0C, 0x07, 0x09, 0x0E, 0x0F, 0x0D, 0x0C, 0x0E, 0x0B, 0x0C, 0x0C, 0x0C, 0xFF, 0xDB, 0x00, 0x43, 0x01, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x06, 0x03, 0x03, 0x06, 0x0C, 0x08, 0x07, 0x08, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0xFF, 0xC0, 0x00, 0x11, 0x08, 0x00, 0x44, 0x00, 0x5A, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01, 0xFF, 0xC4, 0x00, 0x1F, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xFF, 0xC4, 0x00, 0xB5, 0x10, 0x00, 0x02, 0x01, 0x03, 0x03, 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x00, 0x00, 0x01, 0x7D, 0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xA1, 0x08, 0x23, 0x42, 0xB1, 0xC1, 0x15, 0x52, 0xD1, 0xF0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0A, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFF, 0xC4, 0x00, 0x1F, 0x01, 0x00, 0x03, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xFF, 0xC4, 0x00, 0xB5, 0x11, 0x00, 0x02, 0x01, 0x02, 0x04, 0x04, 0x03, 0x04, 0x07, 0x05, 0x04, 0x04, 0x00, 0x01, 0x02, 0x77, 0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71, 0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xA1, 0xB1, 0xC1, 0x09, 0x23, 0x33, 0x52, 0xF0, 0x15, 0x62, 0x72, 0xD1, 0x0A, 0x16, 0x24, 0x34, 0xE1, 0x25, 0xF1, 0x17, 0x18, 0x19, 0x1A, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFF, 0xDA, 0x00, 0x0C, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00, 0xFD, 0xC8, 0xA2, 0x8A, 0x28, 0x00, 0xA2, 0x91, 0xB3, 0x8F, 0x96, 0x94, 0x32, 0xC7, 0x26, 0xD9, 0x3A, 0xE3, 0x3C, 0x50, 0x01, 0x4E, 0x81, 0x15, 0xC4, 0x8D, 0x1E, 0x04, 0xB1, 0xF3, 0xF5, 0xA4, 0x8A, 0x26, 0x25, 0x99, 0x99, 0x76, 0x81, 0x90, 0x33, 0xCD, 0x30, 0x5B, 0xB5, 0xC8, 0x56, 0x66, 0x0A, 0xA3, 0xA0, 0x07, 0x0C, 0xD4, 0x01, 0xE5, 0xFF, 0x00, 0xB5, 0xAF, 0xED, 0x47, 0xA2, 0xFE, 0xC9, 0x3F, 0x0A, 0xAF, 0xBC, 0x63, 0xE2, 0x49, 0xA3, 0x51, 0x60, 0xBF, 0x22, 0x49, 0xCA, 0xB6, 0x41, 0xFF, 0x00, 0x0A, 0xFC, 0x9F, 0xF1, 0x37, 0xFC, 0x1D, 0xC9, 0x63, 0xA7, 0xEB, 0xF3, 0x5A, 0xD8, 0xF8, 0x4F, 0x41, 0xBC, 0x86, 0x2B, 0x96, 0x8C, 0xCA, 0xEE, 0xE0, 0x84, 0x0D, 0x8C, 0xF5, 0xF4, 0xE6, 0xBA, 0xAF, 0xF8, 0x3A, 0xC3, 0xF6, 0x9E, 0xD3, 0x6D, 0x7E, 0x0A, 0x45, 0xE0, 0xAB, 0x3B, 0xC9, 0x23, 0xBE, 0xBE, 0xB7, 0x56, 0xD8, 0x8F, 0x86, 0x3B, 0x49, 0x07, 0xA7, 0x3D, 0xEB, 0xF9, 0xE1, 0x6D, 0x3E, 0xE6, 0x2B, 0xA5, 0xB3, 0x8E, 0x19, 0x2E, 0xE6, 0x98, 0x06, 0x02, 0x05, 0x32, 0x3B, 0x67, 0xB7, 0x1C, 0xE7, 0x9A, 0x00, 0xFE, 0xBD, 0x3F, 0xE0, 0x9D, 0xDF, 0xF0, 0x56, 0xEF, 0x87, 0x3F, 0xB7, 0x9F, 0x85, 0xE3, 0x93, 0x4D, 0xBD, 0xB5, 0x87, 0xC4, 0x0C, 0x9B, 0x9A, 0xD1, 0x06, 0x15, 0x4F, 0x6E, 0x49, 0xAF, 0xAC, 0x81, 0x91, 0x60, 0x6F, 0xB4, 0x8D, 0xB7, 0x1F, 0xC0, 0x83, 0xA1, 0xAF, 0xE3, 0x7B, 0xF6, 0x22, 0xF8, 0x91, 0xF1, 0x53, 0xF6, 0x47, 0xF8, 0xAD, 0xA6, 0xF8, 0x8B, 0x43, 0xF0, 0xC7, 0x8D, 0x63, 0xB6, 0x8E, 0x64, 0x79, 0xD4, 0xE9, 0x77, 0x5E, 0x5B, 0x20, 0xE7, 0x81, 0xB3, 0x06, 0xBF, 0xAC, 0x0F, 0xD8, 0x6F, 0xF6, 0x85, 0xB8, 0xFD, 0xA7, 0xBF, 0x66, 0x5F, 0x0B, 0xF8, 0xB3, 0x51, 0xB3, 0xBC, 0xB0, 0xBC, 0xB9, 0xB3, 0x13, 0x4F, 0x0D, 0xC5, 0xBB, 0x41, 0x28, 0x24, 0x9E, 0x0A, 0xB0, 0x07, 0xF4, 0xA0, 0x0F, 0x60, 0x52, 0x4A, 0xF3, 0xD6, 0x96, 0x91, 0x5C, 0x3A, 0xEE, 0x5C, 0xE0, 0xFA, 0xD2, 0xD0, 0x01, 0x45, 0x0A, 0x77, 0x74, 0xA1, 0x1B, 0xCC, 0x49, 0x19, 0x79, 0x11, 0x8C, 0xB7, 0xB5, 0x00, 0x36, 0x69, 0x1E, 0x38, 0xF7, 0x47, 0xF7, 0xA9, 0x44, 0x4A, 0xD7, 0x4A, 0xF2, 0x7C, 0xDF, 0x2F, 0x22, 0x96, 0xDA, 0x71, 0x21, 0xDD, 0x1F, 0xCD, 0x53, 0x1D, 0x3A, 0x4B, 0x00, 0x65, 0x99, 0x4B, 0x86, 0x1D, 0x0F, 0xBD, 0x00, 0x40, 0xF6, 0xB0, 0x4B, 0x76, 0xD2, 0xC6, 0x77, 0x6D, 0x19, 0x2A, 0x0F, 0x35, 0xC3, 0xFE, 0xD1, 0xBF, 0x1D, 0xFC, 0x3B, 0xFB, 0x39, 0xFC, 0x31, 0xBE, 0xF1, 0x5F, 0x88, 0x75, 0x0B, 0x7B, 0x18, 0xF4, 0xF8, 0x8C, 0xD1, 0x47, 0x2C, 0x9B, 0x1B, 0x8E, 0xB5, 0x81, 0xFB, 0x52, 0x7E, 0xD9, 0x5E, 0x07, 0xFD, 0x8F, 0x3E, 0x1F, 0xDE, 0xEB, 0x9A, 0xFE, 0xA5, 0x67, 0x6B, 0x71, 0x1C, 0x6C, 0x56, 0x29, 0x1B, 0x69, 0x24, 0x0C, 0xE3, 0xF5, 0xAF, 0xE6, 0x8F, 0xFE, 0x0B, 0x17, 0xFF, 0x00, 0x05, 0xA6, 0xF1, 0x4F, 0xED, 0xD9, 0xE3, 0x5B, 0xAD, 0x1F, 0x47, 0xBE, 0xB8, 0xD3, 0xFC, 0x23, 0x1C, 0x8E, 0x81, 0x12, 0x4D, 0xD1, 0x5D, 0x46, 0x73, 0xF5, 0xF5, 0xA0, 0x0F, 0x2D, 0xFF, 0x00, 0x82, 0xB0, 0xFE, 0xDB, 0xD7, 0x9F, 0xB6, 0x77, 0xED, 0x3D, 0xA9, 0x6B, 0xCB, 0x72, 0xCD, 0x63, 0xA4, 0xCD, 0x2D, 0xBD, 0xAE, 0xE3, 0x95, 0x74, 0x24, 0x10, 0x47, 0xAF, 0x4A, 0xA7, 0xFF, 0x00, 0x04, 0x84, 0xD0, 0xBC, 0x3B, 0xE2, 0x1F, 0xDB, 0x17, 0x47, 0x87, 0xC5, 0x5E, 0x50, 0xB3, 0x69, 0x63, 0x64, 0x13, 0x30, 0x0A, 0x5F, 0x78, 0xC7, 0x5A, 0xF9, 0x66, 0xE2, 0xE4, 0xC9, 0x6C, 0x57, 0xD3, 0xBF, 0xAD, 0x74, 0xDF, 0x0A, 0xBC, 0x61, 0x7B, 0xE0, 0xDF, 0x1B, 0x68, 0xDA, 0xC5, 0xAC, 0xCD, 0x1D, 0xD5, 0xBD, 0xE4, 0x38, 0xC1, 0xE4, 0x80, 0xE0, 0xD0, 0x07, 0xF6, 0xCB, 0xE1, 0x4F, 0x83, 0x9E, 0x0F, 0xBC, 0xF0, 0x95, 0x8C, 0x51, 0xE9, 0x5A, 0x44, 0xD6, 0x71, 0xC0, 0x86, 0x07, 0x5B, 0x68, 0xFE, 0x73, 0xB4, 0x77, 0xC5, 0x74, 0x9A, 0x2F, 0x87, 0xEC, 0xFC, 0x3D, 0x68, 0x20, 0x82, 0x18, 0xED, 0x51, 0x46, 0x11, 0x55, 0x42, 0xAC, 0x7F, 0x80, 0xAF, 0x9B, 0x3F, 0xE0, 0x91, 0xBF, 0x1B, 0xEF, 0x3F, 0x68, 0x6F, 0xD8, 0xCB, 0xC3, 0x37, 0xFA, 0x96, 0xE8, 0x6F, 0xA1, 0x52, 0x7E, 0x63, 0x96, 0x38, 0x0A, 0x05, 0x7D, 0x36, 0x8F, 0xF6, 0x8B, 0x96, 0x8E, 0x4F, 0xDE, 0xF3, 0x86, 0xF7, 0xA0, 0x05, 0xFC, 0x43, 0x7B, 0x8E, 0xF4, 0x50, 0xCA, 0x10, 0xE1, 0x46, 0xD0, 0x3A, 0x0A, 0x28, 0x01, 0x93, 0xC5, 0x24, 0x36, 0xF1, 0x48, 0x15, 0x7C, 0xBD, 0xFD, 0x73, 0xC9, 0xAE, 0x2B, 0xF6, 0x90, 0xF8, 0xDD, 0x0F, 0xC0, 0xCF, 0x05, 0xFF, 0x00, 0x6E, 0x5C, 0xDB, 0xFF, 0x00, 0xA1, 0xC6, 0x85, 0xE4, 0xD8, 0xA5, 0x89, 0x03, 0xD8, 0x57, 0x65, 0x7A, 0xDF, 0x65, 0xB3, 0xBC, 0xB8, 0x0D, 0xB9, 0xED, 0xE1, 0x2E, 0x89, 0x9C, 0xE4, 0x8F, 0x6A, 0xFC, 0x7C, 0xFF, 0x00, 0x82, 0x92, 0xFF, 0x00, 0xC1, 0x6C, 0xBE, 0x27, 0x7C, 0x25, 0xF8, 0xBB, 0xA8, 0x78, 0x2F, 0x4C, 0xF0, 0x5D, 0xE6, 0xA1, 0x65, 0x0C, 0xC6, 0x05, 0x69, 0x74, 0x26, 0x9E, 0x36, 0x1C, 0xF7, 0x28, 0x47, 0x6A, 0x00, 0xD1, 0xFD, 0xA6, 0xFF, 0x00, 0xE0, 0xE8, 0x2F, 0x05, 0xF8, 0x22, 0xE2, 0xEF, 0x4F, 0xD1, 0x21, 0x1F, 0xDA, 0x16, 0x24, 0xC6, 0x52, 0x4B, 0x76, 0x8F, 0x73, 0x0A, 0xF8, 0x6B, 0xE3, 0xAF, 0xFC, 0x1D, 0x2B, 0xF1, 0x67, 0xC6, 0x53, 0x4D, 0x6B, 0xA3, 0xE8, 0xFA, 0x74, 0x36, 0x2E, 0x0A, 0xAC, 0xC9, 0x72, 0xEA, 0xC3, 0xB6, 0x71, 0x8A, 0xF9, 0xC3, 0xF6, 0x85, 0xF8, 0x23, 0xF1, 0x73, 0xF6, 0xCD, 0xF1, 0xD5, 0xCF, 0x89, 0x3F, 0xE1, 0x0C, 0x92, 0xC1, 0x65, 0x90, 0xB6, 0xC8, 0x34, 0x96, 0x83, 0x76, 0xEF, 0x60, 0xA2, 0xB2, 0xFC, 0x3D, 0xFF, 0x00, 0x04, 0x65, 0xF8, 0xE1, 0xAF, 0xD8, 0xFD, 0xA2, 0x1D, 0x0E, 0x65, 0x8D, 0xBB, 0x3D, 0xB3, 0x83, 0x40, 0x1E, 0x5B, 0xFB, 0x4B, 0x7E, 0xDE, 0xDF, 0x11, 0x3F, 0x69, 0x5F, 0x10, 0x49, 0x71, 0xAF, 0xF8, 0x93, 0x54, 0x9A, 0xDE, 0x63, 0x96, 0xB5, 0x69, 0xCB, 0xC6, 0xBF, 0x85, 0x78, 0xCD, 0xC0, 0x5B, 0xA9, 0x0B, 0x29, 0x6D, 0x8B, 0xDB, 0xB0, 0xAF, 0xAB, 0xBC, 0x51, 0xFF, 0x00, 0x04, 0x73, 0xF8, 0xCD, 0xE0, 0xFD, 0x26, 0x4B, 0xEB, 0xAD, 0x06, 0xE2, 0x40, 0x80, 0x92, 0x12, 0xD5, 0xCF, 0x4F, 0xC2, 0xBE, 0x74, 0xF8, 0x8B, 0xF0, 0xBF, 0xC4, 0xDF, 0x0F, 0x35, 0x09, 0x6C, 0xF5, 0xAD, 0x26, 0xF2, 0xC1, 0xA3, 0x3B, 0x49, 0x96, 0xDD, 0xA3, 0x07, 0xF3, 0x14, 0x01, 0xCE, 0x5B, 0x8F, 0x36, 0x17, 0x55, 0x19, 0x6C, 0xF1, 0x9A, 0xEA, 0x3E, 0x0F, 0xF8, 0x2B, 0x50, 0xF1, 0xA7, 0x8F, 0x74, 0xAD, 0x3B, 0x4D, 0xB6, 0x92, 0xEE, 0xEE, 0xE2, 0xEA, 0x24, 0x11, 0xAA, 0x92, 0x06, 0x5C, 0x0C, 0xF1, 0x5E, 0x83, 0xFB, 0x36, 0x7E, 0xC3, 0xBE, 0x3E, 0xFD, 0xAB, 0x63, 0x11, 0xF8, 0x57, 0x47, 0xBC, 0x9A, 0x18, 0xE4, 0x58, 0xA4, 0x97, 0xEC, 0xEE, 0x54, 0x13, 0xEE, 0x05, 0x7E, 0xF1, 0x7F, 0xC1, 0x17, 0x7F, 0xE0, 0x80, 0xBA, 0x67, 0xEC, 0xDB, 0xF6, 0x6F, 0x1B, 0x78, 0xE2, 0xD6, 0x3B, 0xAD, 0x55, 0x63, 0x3B, 0x61, 0x66, 0x12, 0x6D, 0x38, 0xC8, 0x3B, 0x5B, 0x3D, 0xF1, 0x40, 0x1F, 0x64, 0xFF, 0x00, 0xC1, 0x1E, 0xFE, 0x06, 0xEA, 0x5F, 0xB3, 0xEF, 0xEC, 0x71, 0xE1, 0xFB, 0x1D, 0x66, 0x36, 0x8E, 0xF6, 0x44, 0x3F, 0x23, 0x1C, 0x91, 0x90, 0xA4, 0x57, 0xD4, 0x91, 0x80, 0x1E, 0x49, 0xBA, 0x34, 0x7C, 0xB0, 0xAA, 0xF6, 0xFB, 0x0E, 0x9B, 0x6E, 0xB6, 0xF0, 0xAC, 0x31, 0xDB, 0x9F, 0x96, 0x35, 0x4D, 0xB8, 0xC7, 0x1D, 0x2A, 0xD5, 0xCC, 0x42, 0x48, 0x7C, 0xC0, 0x70, 0xD3, 0x72, 0xCB, 0x40, 0x0D, 0xDF, 0xE6, 0x7C, 0xDE, 0xB4, 0x52, 0x28, 0xC2, 0xD2, 0xD0, 0x03, 0x26, 0x8A, 0x15, 0x2B, 0x2E, 0xD3, 0xE7, 0x67, 0x93, 0x5C, 0xB7, 0x8A, 0x7E, 0x02, 0xF8, 0x27, 0xE2, 0x26, 0xA8, 0x75, 0x2D, 0x73, 0x45, 0xB7, 0xBA, 0xBD, 0xDD, 0xBF, 0x79, 0x55, 0xCE, 0x7F, 0x2A, 0xEB, 0x38, 0xF4, 0xA1, 0xA1, 0xF3, 0xB8, 0x02, 0x80, 0x39, 0x4B, 0x1F, 0x84, 0xBE, 0x1B, 0xD0, 0x14, 0x0B, 0x1D, 0x26, 0xDE, 0x38, 0xD3, 0xA0, 0xF2, 0xD7, 0xFC, 0x2B, 0xA3, 0xB1, 0xB6, 0x8A, 0xD2, 0xD3, 0xCB, 0x86, 0xDE, 0xD6, 0x35, 0xF4, 0x31, 0x2F, 0xF8, 0x55, 0x8F, 0x35, 0x6D, 0x46, 0xD2, 0xA1, 0xA8, 0x58, 0xD6, 0xE0, 0xFF, 0x00, 0x76, 0x80, 0x29, 0x4F, 0xA3, 0xDB, 0xEB, 0xB6, 0xED, 0x6D, 0x7D, 0x6B, 0x6B, 0x71, 0x03, 0x70, 0x40, 0x89, 0x79, 0xCD, 0x7E, 0x3D, 0x7F, 0xC1, 0xCD, 0xBF, 0xB2, 0xAF, 0x82, 0x7C, 0x39, 0xF0, 0x61, 0x75, 0xCD, 0x2F, 0x48, 0x8E, 0xCF, 0x54, 0x58, 0x64, 0x90, 0x32, 0xE0, 0x17, 0x6E, 0x7D, 0x05, 0x7E, 0xCB, 0x3B, 0x47, 0x62, 0x32, 0xA7, 0x75, 0x7E, 0x75, 0xFF, 0x00, 0xC1, 0x5D, 0x3F, 0xE0, 0x93, 0x3F, 0x10, 0xBF, 0xE0, 0xA3, 0x1A, 0xFE, 0x96, 0xBA, 0x7F, 0x8C, 0x35, 0x7D, 0x17, 0x41, 0x85, 0xDF, 0x7C, 0x10, 0xC4, 0xAF, 0x1B, 0x23, 0x67, 0x8E, 0x54, 0xFA, 0xD0, 0x06, 0x3F, 0xFC, 0x10, 0x2A, 0xEB, 0xE1, 0x9F, 0xC1, 0x9F, 0xD8, 0xE6, 0xD6, 0x38, 0x7C, 0x43, 0xA0, 0xC3, 0xE2, 0x0D, 0x59, 0x60, 0x9E, 0x48, 0x9C, 0xE6, 0x45, 0x6C, 0x10, 0x41, 0x18, 0xAF, 0xD2, 0x0D, 0x32, 0xE9, 0x75, 0x1B, 0x04, 0xBA, 0x8E, 0x68, 0xEE, 0x83, 0x81, 0x89, 0x63, 0x1F, 0x29, 0x15, 0xF8, 0x8B, 0xE0, 0xBF, 0xF8, 0x36, 0x67, 0xE3, 0x07, 0xC3, 0x35, 0x55, 0xD0, 0x7E, 0x2A, 0x78, 0x92, 0xCF, 0xCB, 0x60, 0x63, 0x68, 0xAD, 0xD0, 0x6D, 0x03, 0xFE, 0x01, 0x5F, 0xA6, 0x5F, 0xF0, 0x4E, 0xEF, 0xD9, 0x9F, 0xE2, 0x57, 0xEC, 0xE7, 0xE0, 0x35, 0xD3, 0x7C, 0x77, 0xE2, 0xDD, 0x5B, 0xC4, 0xDE, 0x59, 0x60, 0xAF, 0x78, 0xAA, 0x3D, 0x40, 0xE8, 0x07, 0xB5, 0x00, 0x7D, 0x21, 0x0C, 0x9E, 0x53, 0x96, 0xFE, 0x26, 0xEB, 0x43, 0x62, 0x49, 0x77, 0xFF, 0x00, 0x15, 0x36, 0x5E, 0xB4, 0x89, 0xD6, 0x80, 0x1F, 0x45, 0x14, 0x50, 0x02, 0xC4, 0x7E, 0x73, 0x49, 0xE7, 0x32, 0x4F, 0xC6, 0x28, 0xA2, 0x80, 0x1D, 0x21, 0xCC, 0xCB, 0xD2, 0x96, 0x48, 0x56, 0x45, 0xEE, 0x3E, 0x94, 0x51, 0x40, 0x10, 0x6D, 0xD8, 0xDE, 0xBF, 0x5A, 0x2F, 0xAE, 0x24, 0x58, 0xE3, 0xDB, 0x23, 0xAE, 0xDE, 0x80, 0x1E, 0x28, 0xA2, 0x80, 0x2D, 0x4B, 0x77, 0x2A, 0x68, 0xC7, 0x12, 0x37, 0xCD, 0x8C, 0x9C, 0xF3, 0x55, 0x66, 0x2D, 0x04, 0xAB, 0x1F, 0x99, 0x23, 0x2B, 0x28, 0x63, 0xB8, 0xE7, 0x9A, 0x28, 0xA0, 0x09, 0x23, 0x1B, 0x87, 0x34, 0xAC, 0xBB, 0x4D, 0x14, 0x50, 0x02, 0x51, 0x45, 0x14, 0x01, 0xFF, 0xD9]  )?;

    // Create an "audio" file 
    let mut audio_file = File::create("audio.mp3")?;
    audio_file.write_all(&[0x49, 0x44, 0x33, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x54, 0x53, 0x53, 0x45, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x03, 0x4C, 0x61, 0x76, 0x66, 0x35, 0x39, 0x2E, 0x32, 0x37, 0x2E, 0x31, 0x30, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFB, 0x40, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x6E, 0x66, 0x6F, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x09, 0x48, 0x00, 0x1E, 0x1E, 0x1E, 0x1E, 0x2A, 0x2A, 0x2A, 0x2A, 0x2A, 0x35, 0x35, 0x35, 0x35, 0x35, 0x40, 0x40, 0x40, 0x40, 0x40, 0x4B, 0x4B, 0x4B, 0x4B, 0x57, 0x57, 0x57, 0x57, 0x57, 0x62, 0x62, 0x62, 0x62, 0x62, 0x6D, 0x6D, 0x6D, 0x6D, 0x6D, 0x78, 0x78, 0x78, 0x78, 0x84, 0x84, 0x84, 0x84, 0x84, 0x8F, 0x8F, 0x8F, 0x8F, 0x8F, 0x9A, 0x9A, 0x9A, 0x9A, 0x9A, 0xA5, 0xA5, 0xA5, 0xA5, 0xB1, 0xB1, 0xB1, 0xB1, 0xB1, 0xBC, 0xBC, 0xBC, 0xBC, 0xBC, 0xC7, 0xC7, 0xC7, 0xC7, 0xC7, 0xD2, 0xD2, 0xD2, 0xD2, 0xDE, 0xDE, 0xDE, 0xDE, 0xDE, 0xE9, 0xE9, 0xE9, 0xE9, 0xE9, 0xF4, 0xF4, 0xF4, 0xF4, 0xF4, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x4C, 0x61, 0x76, 0x63, 0x35, 0x39, 0x2E, 0x33, 0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x06, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x48, 0xA8, 0x3A, 0x23, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFB, 0x10, 0xC4, 0x00, 0x00, 0x04, 0x6C, 0x1F, 0x65, 0x54, 0xC0, 0x80, 0x38, 0xAD, 0x89, 0x6C, 0xD3, 0x34, 0x50, 0x00, 0x00, 0x00, 0x59, 0x3C, 0x00, 0x00, 0x00, 0x6E, 0x25, 0x93, 0xDF, 0x38, 0x04, 0x00, 0x80, 0x10, 0x26, 0x2B, 0xBA, 0xF5, 0xEF, 0xDC, 0x3F, 0x00, 0x00, 0x01, 0x1A, 0xF0, 0xF0, 0xF0, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0x00, 0x00, 0x07, 0x95, 0x80, 0xC0, 0xA0, 0x29, 0x50, 0xAD, 0x0A, 0x82, 0x17, 0x56, 0x02, 0xA0, 0x81, 0x4D, 0x65, 0x42, 0xD7, 0x36, 0x3D, 0xF2, 0xE9, 0x99, 0x0C, 0x61, 0x42, 0x48, 0xBE, 0xD7, 0x80, 0xA0, 0x0F, 0x80, 0x22, 0x41, 0xEF, 0xC0, 0x21, 0x10, 0xE8, 0x75, 0x00, 0x00, 0x05, 0x00, 0xFF, 0xFB, 0x12, 0xC4, 0x02, 0x02, 0xC4, 0xFC, 0x21, 0x42, 0x9D, 0xD0, 0x00, 0x28, 0x9F, 0x84, 0x25, 0x81, 0xBF, 0x6C, 0x4D, 0x30, 0x15, 0xCA, 0x60, 0x20, 0x16, 0x38, 0x02, 0x05, 0xC1, 0x23, 0x80, 0xFF, 0x93, 0x64, 0x0E, 0x13, 0x1A, 0x03, 0x93, 0x09, 0x01, 0x33, 0x00, 0xC0, 0xA3, 0x01, 0xC0, 0x63, 0x02, 0xC0, 0x05, 0xDD, 0xDE, 0xD2, 0xB0, 0x90, 0xE6, 0x20, 0x28, 0x66, 0xA7, 0xA6, 0xE9, 0x24, 0x62, 0x4E, 0x3D, 0xA6, 0xFF, 0x9E, 0x96, 0x6F, 0x96, 0x3D, 0xE6, 0x25, 0x81, 0x48, 0x70, 0x6C, 0xE6, 0x7A, 0x8A, 0x66, 0x87, 0x86, 0x6A, 0xEA, 0x63, 0x22, 0xAB, 0x52, 0x5F, 0x60, 0x31, 0x32, 0x92, 0xE3, 0x2C, 0x43, 0x34, 0x57, 0x23, 0xFF, 0xFB, 0x10, 0xC4, 0x04, 0x03, 0xC5, 0x34, 0x23, 0x28, 0x0D, 0xFF, 0x42, 0x60, 0x9F, 0x84, 0x25, 0x81, 0xBF, 0x6C, 0x4D, 0x7E, 0xEF, 0x30, 0x4E, 0xC3, 0x02, 0x32, 0x25, 0x17, 0xD0, 0x32, 0x16, 0xC2, 0xF0, 0x30, 0x47, 0x80, 0xCB, 0x37, 0x6A, 0x0C, 0x81, 0x73, 0x8A, 0xD8, 0xEE, 0x7F, 0x37, 0x06, 0x50, 0x39, 0xF9, 0xB5, 0x8A, 0x44, 0x18, 0x98, 0x89, 0x9B, 0x1C, 0x1B, 0xBC, 0x69, 0x89, 0x70, 0xED, 0x1C, 0x15, 0xF5, 0xC1, 0xBF, 0xC0, 0xEE, 0x18, 0x9A, 0x84, 0xF9, 0xC2, 0x32, 0x19, 0xE2, 0x31, 0x99, 0x9F, 0x99, 0x93, 0xC1, 0x8B, 0x0B, 0x2E, 0xB9, 0x65, 0xB0, 0x35, 0x32, 0x62, 0xF3, 0x2D, 0x44, 0x34, 0x07, 0x73, 0x79, 0xFF, 0xFB, 0x12, 0xC4, 0x04, 0x83, 0xC5, 0x34, 0x23, 0x28, 0x0D, 0xFF, 0x42, 0x60, 0xAE, 0x04, 0x25, 0x01, 0xBF, 0xEC, 0x4D, 0xF4, 0x30, 0x4B, 0x03, 0x15, 0x32, 0x0F, 0xD8, 0x55, 0x32, 0x01, 0x83, 0x03, 0x30, 0x44, 0x40, 0xD2, 0x36, 0x4C, 0x4C, 0xA9, 0x23, 0x8C, 0xCC, 0xEF, 0xF8, 0x37, 0x65, 0x90, 0x35, 0xFA, 0xB3, 0x90, 0x40, 0x19, 0x91, 0x0E, 0x1A, 0x8A, 0x79, 0xD3, 0x5B, 0x18, 0x33, 0x61, 0x2B, 0x99, 0x5B, 0x09, 0x2E, 0x99, 0x52, 0xE1, 0x2C, 0x98, 0x34, 0x00, 0x68, 0x1D, 0xAC, 0x59, 0xAA, 0xB2, 0x9A, 0x42, 0xD1, 0xA3, 0xD4, 0x99, 0x29, 0x3A, 0x74, 0x46, 0xE9, 0x03, 0x04, 0xAA, 0x31, 0x11, 0x63, 0x12, 0x27, 0x32, 0x14, 0xFF, 0xFB, 0x10, 0xC4, 0x03, 0x83, 0xC5, 0x20, 0x23, 0x2E, 0x0D, 0xFF, 0x42, 0x60, 0x9C, 0x84, 0x25, 0x81, 0xAF, 0x6C, 0x4D, 0x83, 0x44, 0xA1, 0x30, 0x2E, 0x42, 0x16, 0x31, 0x53, 0x90, 0xC4, 0x31, 0x4A, 0xC2, 0x0C, 0x30, 0x29, 0x00, 0x8B, 0x32, 0x66, 0x80, 0x43, 0x4D, 0x49, 0x63, 0x71, 0x14, 0xCF, 0x05, 0x57, 0xD2, 0xDB, 0x5C, 0x49, 0x03, 0x2E, 0x34, 0xDD, 0x30, 0x3F, 0xBF, 0xCC, 0x4E, 0x86, 0xC8, 0xE1, 0xFF, 0x60, 0x8E, 0x12, 0x86, 0xD0, 0xC4, 0xFC, 0x25, 0x4E, 0x19, 0x88, 0xCE, 0x92, 0x0C, 0xBD, 0x04, 0xCB, 0x1F, 0x8C, 0x48, 0x61, 0x75, 0xC6, 0x2B, 0x81, 0xD5, 0x31, 0x30, 0xBC, 0xC5, 0x24, 0x63, 0x1D, 0x1F, 0x4C, 0xFF, 0xFB, 0x12, 0xC4, 0x04, 0x83, 0xC5, 0x50, 0x23, 0x28, 0x0E, 0x7F, 0x42, 0x60, 0x96, 0x84, 0x25, 0xC1, 0xAF, 0x68, 0x4D, 0xDB, 0x00, 0x30, 0x44, 0x03, 0x35, 0x31, 0xE2, 0x59, 0x45, 0x31, 0xD5, 0x03, 0x23, 0x30, 0x3C, 0xC0, 0xE0, 0x34, 0x4F, 0x4C, 0xE8, 0x63, 0x98, 0xAC, 0xF2, 0xEC, 0x37, 0xE4, 0x90, 0x86, 0x06, 0xB3, 0xD4, 0xCF, 0x32, 0x01, 0x0D, 0x8A, 0x53, 0xD9, 0x90, 0xC4, 0x5C, 0x59, 0xCD, 0xE5, 0x2D, 0x2C, 0xDD, 0x94, 0x5A, 0xCC, 0x46, 0x01, 0xE0, 0xF9, 0xD3, 0x35, 0xEF, 0x4D, 0x13, 0x63, 0x43, 0xBC, 0xC5, 0x93, 0x6B, 0x94, 0xF8, 0x06, 0x2A, 0x30, 0xB1, 0x03, 0x0A, 0x1F, 0x31, 0x53, 0xE3, 0x30, 0x8F, 0x30, 0x8D, 0xFF, 0xFB, 0x10, 0xC4, 0x06, 0x02, 0xC4, 0x9C, 0x1F, 0x30, 0x0D, 0xFB, 0x22, 0x68, 0x89, 0x03, 0xA7, 0x61, 0x9F, 0x64, 0x4D, 0x1E, 0x43, 0x43, 0x4F, 0x3A, 0x34, 0x17, 0x1D, 0xA3, 0x07, 0xA0, 0x7D, 0x30, 0xDA, 0x0C, 0x34, 0xE3, 0x28, 0xF0, 0xC4, 0x05, 0xAA, 0xFE, 0x9D, 0x18, 0x60, 0x3D, 0x89, 0x5E, 0x02, 0x30, 0xE3, 0x6C, 0xC1, 0xC8, 0x22, 0x8C, 0xDE, 0x53, 0x60, 0xCD, 0x88, 0x23, 0x0C, 0x1C, 0xC0, 0x94, 0xE4, 0x50, 0xC4, 0x30, 0x02, 0x48, 0x12, 0x24, 0x4C, 0x97, 0x81, 0x81, 0xD5, 0x30, 0x81, 0x13, 0x09, 0x20, 0x31, 0x24, 0x13, 0x2D, 0x92, 0x30, 0x82, 0x1E, 0xD3, 0x3F, 0x9F, 0x77, 0x33, 0xDC, 0x1E, 0x33, 0x07, 0x10, 0xFF, 0xFB, 0x12, 0xC4, 0x0B, 0x82, 0xC4, 0xAC, 0x23, 0x30, 0x0D, 0xFB, 0x22, 0x60, 0x88, 0x83, 0xA7, 0x15, 0x9F, 0x64, 0x4F, 0x80, 0x02, 0x3E, 0x24, 0xB1, 0xC8, 0x49, 0xE7, 0x60, 0x1B, 0x65, 0xFB, 0x3D, 0x67, 0xA8, 0x6C, 0xE8, 0x50, 0x66, 0x82, 0x76, 0x40, 0x61, 0x12, 0x13, 0x26, 0x80, 0xCA, 0x6E, 0x67, 0xDA, 0x13, 0x66, 0x11, 0x00, 0x58, 0x77, 0xB6, 0x64, 0xA2, 0x61, 0x1C, 0x60, 0xDA, 0x5E, 0xF8, 0xC0, 0x61, 0x65, 0x30, 0xA2, 0x8C, 0x21, 0x93, 0x16, 0xD8, 0xCF, 0x7F, 0x30, 0x67, 0x1B, 0x23, 0x34, 0x2D, 0xC0, 0x33, 0x2D, 0x1A, 0xA3, 0x05, 0x60, 0x71, 0x10, 0xB4, 0x3C, 0x69, 0xBE, 0x01, 0xD4, 0xF8, 0x3A, 0x96, 0x85, 0x3A, 0xFF, 0xFB, 0x10, 0xC4, 0x11, 0x82, 0xC4, 0x74, 0x1F, 0x32, 0x0D, 0x7B, 0x22, 0x68, 0x87, 0x03, 0xA7, 0x15, 0x9F, 0x64, 0x4F, 0xB2, 0x1C, 0xD4, 0x2B, 0x01, 0x30, 0x77, 0xBA, 0x61, 0x14, 0x11, 0x86, 0x82, 0xA9, 0xC4, 0x67, 0xF4, 0x12, 0x06, 0x11, 0x60, 0x52, 0x77, 0x34, 0x64, 0x24, 0x60, 0xA2, 0x00, 0xB8, 0xBD, 0x91, 0xB0, 0x32, 0x2A, 0x30, 0x40, 0xB3, 0x03, 0x18, 0x30, 0x83, 0x73, 0x1B, 0x80, 0x30, 0x5E, 0x1B, 0x93, 0x30, 0xBD, 0xED, 0x32, 0xF4, 0x1B, 0x13, 0x04, 0xD0, 0x72, 0x18, 0x74, 0x99, 0x20, 0x17, 0xA7, 0x5B, 0x41, 0x15, 0x34, 0x19, 0xE5, 0x50, 0xD0, 0x91, 0x10, 0x0C, 0xF9, 0xDC, 0xC1, 0x84, 0x68, 0x41, 0x9A, 0xFF, 0xFB, 0x12, 0xC4, 0x18, 0x02, 0xC4, 0x88, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0x87, 0x03, 0xA7, 0x15, 0x9F, 0x64, 0x4F, 0x10, 0x24, 0x59, 0xA0, 0x38, 0x42, 0x98, 0x46, 0x01, 0x21, 0xDE, 0xC9, 0x92, 0x98, 0x01, 0x00, 0x2D, 0xE8, 0x0B, 0x8C, 0x06, 0x16, 0x30, 0x30, 0xA3, 0x00, 0x19, 0x30, 0x73, 0x83, 0x16, 0x83, 0x30, 0x55, 0x1B, 0xE3, 0x2D, 0x0E, 0x14, 0x32, 0xBB, 0x1B, 0x63, 0x04, 0x40, 0x75, 0x2A, 0xBC, 0x50, 0xA0, 0x1C, 0x23, 0xA9, 0x70, 0xCA, 0x5A, 0x14, 0xEA, 0xC8, 0x66, 0xA8, 0x46, 0x0A, 0x70, 0xEF, 0x54, 0xC2, 0x38, 0x1D, 0x8D, 0x0B, 0x50, 0xB8, 0xD0, 0x48, 0x1E, 0x0C, 0x23, 0xC0, 0x8C, 0xEE, 0x60, 0xC6, 0x48, 0xFF, 0xFB, 0x10, 0xC4, 0x1F, 0x02, 0xC4, 0x8C, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0x87, 0x03, 0xA7, 0x15, 0x9F, 0x64, 0x4F, 0x02, 0x88, 0x23, 0x04, 0x04, 0x4A, 0xC0, 0xC8, 0xD5, 0x30, 0x20, 0xB3, 0x01, 0x1A, 0x30, 0x23, 0x93, 0x13, 0x84, 0x30, 0x4A, 0x1C, 0x33, 0x29, 0x3E, 0x39, 0x32, 0x7E, 0x1B, 0xD3, 0x03, 0x90, 0x76, 0x19, 0x84, 0x79, 0xB0, 0x78, 0x27, 0x6A, 0x41, 0x97, 0x34, 0x19, 0xE5, 0x50, 0xBD, 0x86, 0x84, 0x05, 0x56, 0x7D, 0x2C, 0x61, 0x4E, 0x0F, 0xA6, 0x9A, 0x08, 0xB6, 0x69, 0x62, 0x10, 0x26, 0x14, 0xC0, 0x4C, 0x7C, 0xBE, 0x65, 0xB2, 0x60, 0x2C, 0x05, 0xF4, 0x42, 0x0C, 0xA0, 0x30, 0xB2, 0x30, 0x30, 0xA0, 0xFF, 0xFB, 0x12, 0xC4, 0x25, 0x02, 0xC4, 0x88, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0x89, 0x83, 0xA6, 0xD5, 0x9F, 0x64, 0x4F, 0x00, 0xD9, 0x80, 0x9C, 0x18, 0x74, 0x29, 0x82, 0x08, 0xE4, 0x19, 0x29, 0x72, 0xC1, 0x92, 0x08, 0xE0, 0x18, 0x17, 0x03, 0xD8, 0x82, 0x41, 0x67, 0x02, 0x46, 0x3B, 0x0F, 0x0E, 0xB5, 0xA1, 0x4E, 0xAC, 0x85, 0xEA, 0x89, 0x61, 0x10, 0x1E, 0x26, 0x98, 0x48, 0x02, 0xE1, 0xA1, 0xF9, 0xB6, 0x1A, 0x12, 0x03, 0x01, 0x84, 0x88, 0x0D, 0x9D, 0xCA, 0x18, 0xC8, 0x02, 0x51, 0x0A, 0x60, 0x82, 0x09, 0x58, 0x1D, 0x35, 0x30, 0x00, 0xB0, 0x28, 0xD0, 0x00, 0xEC, 0xC2, 0xE1, 0x8C, 0x0D, 0x87, 0x2C, 0xC8, 0x47, 0x9C, 0xCC, 0xFF, 0xFB, 0x10, 0xC4, 0x2B, 0x82, 0xC4, 0x7C, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0x86, 0x03, 0xA7, 0x15, 0x9F, 0x64, 0x4F, 0x7F, 0x87, 0x14, 0xC0, 0x8C, 0x1E, 0x81, 0xB3, 0x89, 0x3E, 0x1E, 0x29, 0xDE, 0x40, 0x95, 0xCD, 0x46, 0x79, 0x54, 0x2A, 0x1E, 0x30, 0x06, 0x9E, 0x75, 0x8C, 0x18, 0x72, 0x84, 0x19, 0xB2, 0xF2, 0x40, 0x9B, 0x1B, 0x84, 0x49, 0x87, 0x10, 0x18, 0x1D, 0x35, 0xA6, 0x55, 0x39, 0x84, 0x50, 0x05, 0xB2, 0x21, 0x17, 0x0C, 0x57, 0xA5, 0x30, 0x10, 0xA0, 0x20, 0xD8, 0x14, 0xE8, 0xC1, 0x22, 0x4C, 0x0A, 0xC7, 0x38, 0xC7, 0x23, 0xA2, 0x8C, 0x6E, 0xC7, 0x18, 0xC0, 0x58, 0x1F, 0x40, 0x14, 0x06, 0x40, 0x26, 0x51, 0xFF, 0xFB, 0x12, 0xC4, 0x32, 0x03, 0xC4, 0x70, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0x8D, 0x03, 0xE6, 0x41, 0xAF, 0x68, 0x4D, 0xDC, 0x28, 0xB5, 0xAD, 0x4A, 0x89, 0x60, 0x51, 0x09, 0x12, 0xB9, 0xAA, 0x03, 0x1D, 0x39, 0x99, 0x8C, 0xC0, 0x66, 0x1D, 0x6D, 0xB4, 0xA1, 0xD4, 0xB0, 0x6C, 0x18, 0xCA, 0x82, 0xE9, 0xCF, 0x2D, 0x19, 0xA2, 0xB1, 0x8B, 0xAB, 0x98, 0x55, 0x40, 0x20, 0x99, 0xA5, 0xCC, 0x57, 0x22, 0x30, 0x00, 0x90, 0x48, 0xD0, 0x30, 0xEC, 0xC0, 0x62, 0x0C, 0x08, 0x07, 0x44, 0xC5, 0xFF, 0xA7, 0x4C, 0x5C, 0x07, 0x2C, 0xC0, 0x2C, 0x1F, 0x8C, 0x3A, 0xC1, 0xD3, 0x8D, 0x92, 0x03, 0x78, 0x6A, 0xE6, 0xA3, 0x46, 0xA9, 0x90, 0x83, 0xFF, 0xFB, 0x10, 0xC4, 0x38, 0x83, 0xC4, 0x70, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0x9D, 0x84, 0x25, 0x41, 0xBF, 0x6C, 0x4D, 0x13, 0x40, 0x9B, 0xB8, 0x49, 0xF1, 0x1C, 0x19, 0x0E, 0x87, 0x09, 0xF1, 0x53, 0x8B, 0x9E, 0xE9, 0x87, 0x99, 0x90, 0xB0, 0x39, 0x1E, 0xAB, 0xD9, 0xA5, 0xBB, 0x99, 0x13, 0xD1, 0x87, 0xF2, 0x82, 0x4F, 0x56, 0x02, 0x3F, 0x52, 0xBE, 0x95, 0x00, 0x84, 0x03, 0x06, 0xC2, 0xA7, 0x40, 0x88, 0x93, 0x01, 0x31, 0xD2, 0x31, 0x2E, 0xEA, 0xC3, 0x12, 0x31, 0xCC, 0x04, 0x04, 0x09, 0x8D, 0x60, 0x2A, 0x82, 0x32, 0x81, 0xEE, 0x8F, 0x5A, 0xD4, 0xA8, 0x96, 0x32, 0x20, 0x91, 0x68, 0x60, 0x37, 0x81, 0xF2, 0x93, 0x99, 0xFF, 0xFB, 0x12, 0xC4, 0x3C, 0x83, 0xC4, 0x74, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0xA5, 0x84, 0xA4, 0xC1, 0xBF, 0x6C, 0x4D, 0x0F, 0x05, 0x61, 0xF1, 0xCA, 0xBF, 0x1E, 0xEF, 0x06, 0x01, 0x90, 0xB8, 0x33, 0x9E, 0x93, 0xA1, 0xA4, 0x3B, 0x18, 0xEB, 0xD9, 0x83, 0x73, 0x05, 0x0F, 0x16, 0x5C, 0x72, 0x6E, 0xA6, 0xD6, 0x01, 0x00, 0x00, 0x00, 0x24, 0x2A, 0x34, 0x20, 0x3B, 0x0B, 0xC5, 0x18, 0x02, 0x0E, 0xA9, 0x86, 0xBF, 0x5B, 0x98, 0x66, 0x0E, 0x79, 0x28, 0x41, 0x19, 0xB7, 0x81, 0xAB, 0x29, 0x30, 0x3D, 0xE2, 0x2C, 0x9A, 0x8D, 0x1D, 0xE5, 0x8C, 0x8C, 0x30, 0x22, 0x24, 0x3B, 0xCC, 0xF9, 0x05, 0x0C, 0x87, 0xC1, 0xCC, 0xF9, 0x00, 0xEE, 0xFF, 0xFB, 0x10, 0xC4, 0x40, 0x03, 0xC4, 0x40, 0x1F, 0x32, 0x0D, 0xFB, 0x22, 0x68, 0xA4, 0x04, 0xA4, 0xC1, 0xBF, 0x6C, 0x4D, 0x4F, 0x79, 0xC2, 0x2C, 0xC8, 0x58, 0x16, 0x0F, 0x45, 0xC4, 0xD1, 0x5D, 0x4C, 0x59, 0xE8, 0x03, 0xCE, 0x2A, 0x7A, 0xB6, 0x24, 0xB4, 0x15, 0xF4, 0xAD, 0x30, 0x10, 0xC0, 0x61, 0x18, 0x85, 0x18, 0x52, 0x84, 0x2A, 0x10, 0xD1, 0x81, 0x0C, 0x86, 0x61, 0x80, 0xFE, 0x10, 0xA0, 0x24, 0x09, 0x93, 0x5B, 0xA3, 0x70, 0x41, 0xF8, 0xC6, 0xF5, 0x26, 0xF5, 0x4E, 0x68, 0xAF, 0xAA, 0x64, 0x81, 0xE0, 0x68, 0xA2, 0x6F, 0x10, 0x1E, 0x59, 0x91, 0x00, 0x17, 0x1F, 0x22, 0x8C, 0xB1, 0xEF, 0x40, 0x26, 0x19, 0x0B, 0x82, 0xFF, 0xFB, 0x12, 0xC4, 0x44, 0x02, 0xC4, 0xA8, 0x21, 0x32, 0xAD, 0xFB, 0x22, 0x68, 0xA1, 0x84, 0xA4, 0xC1, 0xBF, 0x6C, 0x4D, 0x49, 0xE7, 0x35, 0x1A, 0x03, 0x91, 0x88, 0xBD, 0x85, 0xB9, 0x89, 0x0F, 0x14, 0x7E, 0x49, 0x43, 0x73, 0x6B, 0x2A, 0x00, 0x00, 0x1A, 0xC0, 0x00, 0x0C, 0x0D, 0x00, 0x41, 0x50, 0x98, 0x1F, 0x84, 0x99, 0x85, 0x18, 0x48, 0x98, 0x8A, 0x90, 0x69, 0x8D, 0x17, 0xF5, 0x98, 0xC4, 0x11, 0x39, 0x89, 0x48, 0x4E, 0x98, 0x49, 0x82, 0x49, 0x81, 0x68, 0x2A, 0x98, 0x1B, 0x81, 0xF8, 0x08, 0x20, 0x86, 0x80, 0xED, 0x2B, 0x1C, 0x88, 0xA7, 0xED, 0x3D, 0xDF, 0x44, 0xA0, 0x02, 0x37, 0x78, 0x89, 0x40, 0x00, 0x09, 0xDE, 0x04, 0xFF, 0xFB, 0x10, 0xC4, 0x47, 0x03, 0xC4, 0x7C, 0x21, 0x30, 0x0D, 0xFF, 0x22, 0x68, 0xA1, 0x04, 0xA4, 0xC1, 0xBF, 0x6C, 0x4D, 0x10, 0x42, 0x20, 0x00, 0x12, 0xA1, 0x4E, 0x16, 0xA1, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAD, 0x80, 0xA0, 0x19, 0xA6, 0xBD, 0xFE, 0xAB, 0xAD, 0xD0, 0xFF, 0xF9, 0xF1, 0xEC, 0x9B, 0x98, 0xFB, 0x49, 0x44, 0xDF, 0xF3, 0x2B, 0x97, 0x3C, 0x3A, 0xB5, 0x6E, 0x71, 0xFF, 0xCF, 0xA5, 0xB4, 0xCF, 0x17, 0x4E, 0xBE, 0xD5, 0xD9, 0xAD, 0x7F, 0xFF, 0xCE, 0x7D, 0x34, 0xD5, 0x82, 0x4D, 0x65, 0xBC, 0xDB, 0x4D, 0x61, 0x99, 0x54, 0xBA, 0xAD, 0x4C, 0x41, 0x4D, 0x45, 0x33, 0x2E, 0x31, 0x30, 0x30, 0x55, 0x55, 0x55, 0xFF, 0xFB, 0x12, 0xC4, 0x4A, 0x00, 0x08, 0x4C, 0x5B, 0x31, 0x35, 0xE1, 0x80, 0x21, 0x15, 0x0A, 0x6E, 0xFB, 0x37, 0xB0, 0x02, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0xFF, 0xFB, 0x10, 0xC4, 0x30, 0x03, 0xC0, 0x00, 0x01, 0xA4, 0x1C, 0x00, 0x00, 0x20, 0x00, 0x00, 0x34, 0x80, 0x00, 0x00, 0x04, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55])?;

    // Create a "zip" file 
    let mut zip_file = File::create("archive.zip")?;
    zip_file.write_all(&[0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x00, 0x08, 0x00, 0x5A, 0x1D, 0xE9, 0x58, 0x62, 0xFC, 0x4D, 0x0E, 0x44, 0x00, 0x00, 0x00, 0x57, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x74, 0x65, 0x78, 0x74, 0x2E, 0x74, 0x78, 0x74, 0x15, 0xC2, 0xC1, 0x11, 0x80, 0x20, 0x0C, 0x04, 0xC0, 0x56, 0xAE, 0x02, 0xFA, 0xF0, 0x6F, 0x03, 0x67, 0x8C, 0x92, 0x11, 0x91, 0x21, 0x79, 0x58, 0xBE, 0x66, 0x67, 0xD7, 0x6A, 0x8E, 0x3F, 0xE1, 0xBC, 0x47, 0x53, 0x84, 0xBE, 0x81, 0xC3, 0x9A, 0x16, 0x2C, 0x01, 0x79, 0x7A, 0xD0, 0xBA, 0x63, 0xEA, 0x50, 0x86, 0xF5, 0x13, 0x52, 0x39, 0x29, 0xA1, 0xD3, 0xD1, 0xEC, 0x52, 0x30, 0x6D, 0x49, 0xD2, 0x9E, 0xCA, 0x07, 0x50, 0x4B, 0x01, 0x02, 0x14, 0x00, 0x14, 0x00, 0x00, 0x00, 0x08, 0x00, 0x5A, 0x1D, 0xE9, 0x58, 0x62, 0xFC, 0x4D, 0x0E, 0x44, 0x00, 0x00, 0x00, 0x57, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x65, 0x78, 0x74, 0x2E, 0x74, 0x78, 0x74, 0x50, 0x4B, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x36, 0x00, 0x00, 0x00, 0x6A, 0x00, 0x00, 0x00, 0x00, 0x00])?;


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
 