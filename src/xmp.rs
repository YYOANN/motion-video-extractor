
use xmp_toolkit::XmpMeta;
use std::str::from_utf8;
use std::str::FromStr;

/// Adobe XMP delimiter used to indicate the start
/// of XMP data inside the `APP1` section of a `JFIF` (JPEG) file.
pub static XMP_DELIMITER: &[u8] = b"http://ns.adobe.com/xap/1.0/\0";

pub fn decode_xmp_data(data: &[u8]) -> Option<XmpMeta> {
    from_utf8(&data).map_or(None, |xmp_data_str| XmpMeta::from_str(&xmp_data_str[XMP_DELIMITER.len()..]).ok())
}

