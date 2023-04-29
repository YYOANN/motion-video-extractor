

# Motion Video Extractor

The aim of this project is to extract short embedded videos (1 to 2 seconds)
from photos taken with a Google Pixel phone with enabled feature (usually the
filename will end with `.MP.jpg`). Traditional methods used a brute-force
approach by scanning the file to detect the beginning of the `MP4` section.
While effective, this method is slow and may not work if the `JPEG` file
contains more than one embedded file.

Google stores the metadata information required to extract the sidecar video
file inside a `JFIF` `APP1` section that contains `XMP` metadata. This project
decodes the beginning of the file and parses the `XMP` section to extract
relevant information (video `length` and `padding`) and then copies the section
into a new file with the same name as the original `JPEG` file with an extra
`.mp4` extension. This method is fast and the read/write speed of the disk will
be the main bottleneck.

# Build

This is a Rust project. If you have not yet installed Rust on you computer you
can do it using rustup project `https://rustup.rs/`.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You can then open a shell at the root of the project and compile it with `cargo`.

```sh
cargo build --release
```

The result binary file should be in the directory
`./target/release/motion-photo-extractor`


# Usage

To extract the sidecar video from specific files, simply run the binary and
provide the filenames as arguments.

```sh
../motion-photo-extractor file_001.jpg file_002.jpg file_003.jpg
```


# Future improvements

## Support more manufacturer

At the moment, the project only supports the Google Pixel Motion Photo feature.
However, other manufacturers are also embedding sidecar files inside their
`JPEG` files. If you have any sample files, I would be glad to work on finding a
way to extract the short videos contained within them"

## Add CI/CD

It would be beneficial to add continuous integration and binary build
functionality, which would allow users to download the binary directly from
GitHub.
