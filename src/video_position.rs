use std::fs;
use std::io::{Seek, SeekFrom, Read};

/// Basic struct to hold the position
/// of a sidecar video file in in Motion Picture
/// photo
#[derive(Debug, PartialEq)]
pub struct VideoPosition {
    from_eof: u64,
    padding: u64,
}

#[derive(Debug, Clone)]
struct DoubleError;

impl VideoPosition {
    /// Constructor for `VideoPosition`
    ///
    /// Arguments:
    ///   - `from_eof`: `u64` number of bytes from start of
    ///     sidecar file up to the end of the file
    ///   - `padding`: `u64` number of bytes from end of
    ///     sidecar file up to the end of the file
    pub fn new(from_eof: u64, padding: u64) -> Self {
        VideoPosition { from_eof, padding }
    }

    /// Return the computed video length in bytes
    /// May return `None` if `padding` value is
    /// superior to the `from_eof` value
    pub fn video_length(self: &Self) -> Option<u64> {
        if self.from_eof <= self.padding {
            None
        } else {
            Some(self.from_eof - self.padding)
        }
    }

    /// Return how many bytes are present from start of sidecar video
    /// up to the end of the file
    pub fn from_eof(self: &Self) -> u64 {
        self.from_eof
    }

    /// Return how many bytes are present after the video sidecar
    /// file up to the end of the file
    pub fn padding(self: &Self) -> u64 {
        self.padding
    }

    /// Use offsets on file `filename` to extract the relevant
    /// portion of the file and write it to a new file with the
    /// added `.mp4` extension
    pub fn extract_video_from_file(self: &Self, filename: &str) {
        // TODO: Split function, make it more generic and return success or custom error
        let file_size = fs::metadata(filename).map_or(0, |f| f.len());
        let video_length = self.video_length();

        if self.from_eof() > file_size {
            println!("Invalid offset (larger than file size) for \"{:?}\"", filename);
            return
        }

        if None == video_length {
            println!("Invalid VideoPosition ({:?}) for \"{:?}\" of size {}", self, filename, file_size);
            return
        }

        println!("Processing file \"{}\"", filename);
        let mut file_for_copy = {
            match fs::File::open(filename) {
                Ok(file) => file,
                Err(err) => {
                    println!("Can't open file \"{}\" to extract video: {:?}", filename, err);
                    return
                }
            }
        };

        let mut read_buffer = Vec::with_capacity(self.from_eof() as usize);
        file_for_copy.seek(SeekFrom::Start(file_size - self.from_eof())).unwrap();
        file_for_copy.take(video_length.unwrap()).read_to_end(&mut read_buffer).unwrap();

        let mut video_filename = filename.to_string();
        video_filename.push_str(".mp4");
        let _ = fs::write(&video_filename, read_buffer);
    }
}

#[cfg(test)]
mod tests {
    use crate::video_position::VideoPosition;
    #[test]
    fn test_video_position() {
        let valid_vp = VideoPosition::new(500, 100);
        let invalid_vp = VideoPosition::new(500, 500);
        assert_eq!(valid_vp.from_eof(), 500);
        assert_eq!(valid_vp.padding(), 100);
        assert_eq!(valid_vp.video_length(), Some(400));
        assert_eq!(invalid_vp.video_length(), None);
    }
}
