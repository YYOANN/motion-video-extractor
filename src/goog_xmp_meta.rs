

/// Google namespace used of XMP metadata in photos
static GOOGLE_NAMESPACE: &str = "http://ns.google.com/photos/1.0/container/";

use xmp_toolkit::XmpMeta;
use std::str::FromStr;
use crate::video_position::VideoPosition;

/// Some functions to access Google XMP information
pub trait GoogXmpMeta {
    /// Return the `Some(value)` of a property if at a given index for a
    /// Google `Directory` (array) container in XMP Data or `None` if not found
    fn get_goog_property(self: &Self, index: usize, item: &str) -> Option<String>;

    /// Return the `Some(value)` of the *MIME* property of an `Item` inside a `Directory` (array)
    /// at the given index or None if not found
    fn get_goog_mime_type_property(self: &Self, index: usize) -> Option<String> {
        self.get_goog_property(index, "Mime")
    }

    /// Return the `Some(value)` of the *Length* property of an `Item` inside a `Directory` (array)
    /// at the given index or None if not found
    fn get_goog_length_property(self: &Self, index: usize) -> Option<u64> {
        self.get_goog_property(index, "Length").and_then(|length_str| u64::from_str(&length_str).ok())
    }

    /// Return the `Some(value)` of the *Padding* property of an `Item` inside a `Directory` (array)
    /// at the given index or None if not found
    fn get_goog_padding_property(self: &Self, index: usize) -> Option<u64> {
        self.get_goog_property(index, "Padding").and_then(|padding| u64::from_str(&padding).ok())
    }

    /// Return the `Some(VideoPosition)` if extraction from XMP data is successful
    /// otherwise return None
    fn video_position_from_xmp(self: &Self) -> Option<VideoPosition>;
}

impl GoogXmpMeta for XmpMeta {
    fn get_goog_property(self: &Self, index: usize, item: &str) -> Option<String> {
        let item_path = format!("Container:Directory[{}]/Container:Item/Item:{}", index.to_string(), item);
        self.property(GOOGLE_NAMESPACE, &item_path).map(|item| item.value)
    }

    fn video_position_from_xmp(self: &Self) -> Option<VideoPosition> {
        // FIXME: Currently only check from indexes from 0 to 10
        // TODO: use property_array seems to only support string properties for now
        for i in 0..10 {
            if self.get_goog_mime_type_property(i) == Some("video/mp4".to_string()) {
                let padding = self.get_goog_padding_property(i).unwrap_or(0);
                return self.get_goog_length_property(i).map(|length| {
                    VideoPosition::new(length + padding, padding)
                })
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::video_position::VideoPosition;
    use std::fs;
    use std::io::Read;
    use crate::xmp::decode_xmp_data;
    use crate::goog_xmp_meta::GoogXmpMeta;

    #[test]
    fn test_goog_xmp() {
        let mut xmp_data = vec![];
        let mut xmp_file = fs::File::open("sample/sample.xmp").unwrap();
        let _ = xmp_file.read_to_end(&mut xmp_data);
        let Some(xmp) = decode_xmp_data(&xmp_data) else { panic!("Unable to decode sample.xmp XMP data"); };

        assert_eq!(xmp.video_position_from_xmp(), Some(VideoPosition::new(1199111, 0)));
    }
}
