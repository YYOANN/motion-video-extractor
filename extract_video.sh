#!/usr/bin/env bash

# POC for video extraction via XMP RDF metadata
# Tools needed:
#  - exiftool: to extract XMP data from file
#  - xq: To convert XMP XML data to JSON
#  - jq: To access the legth value we need
#  - bash / cat /tail

get_offset () {
    local filename="$1"
    exiftool -xmp -b  "$filename" \
        | xq -j \
        | jq -r -f <(cat <<EOF
."x:xmpmeta"
."rdf:RDF"
."rdf:Description"
."Container:Directory"
."rdf:Seq"
."rdf:li"
| map(select(."Container:Item"."@Item:Mime"=="video/mp4"))
| .[0]
| ."Container:Item"
  ."@Item:Length"
EOF
)
}

process_file () {
    local filename="$1"
    local offset="$2"
    cat "$filename" | tail -c +$((`stat -c %s "$filename" ` - $offset + 1)) > "$filename.mp4"
}


for file in "$@"
do
    echo $file
    process_file "$file" `get_offset "$file"`
done

