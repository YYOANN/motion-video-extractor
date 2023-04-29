use jfifdump::SegmentKind;
use std::fs;
use std::io::BufReader;
use xmp_toolkit::XmpMeta;
use std::env::args;
use motion_photo_extractor::goog_xmp_meta::GoogXmpMeta;
use motion_photo_extractor::xmp::{XMP_DELIMITER, decode_xmp_data};

fn process_file(filename: &str) {
    let mut jifi_reader = {
        match fs::File::open(filename) {
            Ok(file) => {
                match jfifdump::Reader::new(BufReader::new(file)) {
                    Ok(jifi_reader) => jifi_reader,
                    Err(err) => {
                        println!("Can't create jifi reader for file \"{}\": {:?}", filename, err);
                        return
                    }
                }
            }
            Err(err) => {
                println!("Can't open file \"{}\": {:?}", filename, err);
                return
            }
        }
    };


    // Loop through JIFI image segments
    loop {
        match jifi_reader.next_segment().unwrap().kind {
            SegmentKind::Eoi => break,
            SegmentKind::App{nr:1, data} => {
                // TODO: EXIF APP1 data for other manufacturers
                // XMP APP1 data
                if data.starts_with(XMP_DELIMITER) {
                    let offset_opt = decode_xmp_data(&data).and_then(|xmp_data| {
                        // TODO: Add support for other manufacturers
                        <XmpMeta as GoogXmpMeta>::video_position_from_xmp(&xmp_data)
                    });
                    if let Some(position) = offset_opt {
                        position.extract_video_from_file(&filename);
                        break;
                    } else {
                        println!("No video position found in XMP for file \"{:?}\"", filename);
                    }

                }
            },
            _ => (),
        }
    }
}

fn main() {
    for arg in args().skip(1) {
        process_file(&arg);
    }
}
