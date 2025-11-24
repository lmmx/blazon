# Generate dependency and binary size badges
blazon-md:
    #!/usr/bin/env bash
    set -euo pipefail

    # Get dependency count
    dep_count=$(cargo tree --edges normal --prefix none | sort -u | wc -l)

    # Get binary size (build if needed)
    binary_name="blazon"
    if [ ! -f "target/release/$binary_name" ] || [ "Cargo.toml" -nt "target/release/$binary_name" ]; then
        cargo build --release --quiet --features cli
    fi

    # Get size in bytes and format
    size=$(stat -c%s "target/release/$binary_name" 2>/dev/null || stat -f%z "target/release/$binary_name")
    size_mb=$(echo "scale=1; $size / 1048576" | bc)

    # Generate inline badge markdown with shields.io
    crates_io_url='https://crates.io/crates/blazon'

    deps_img="https://img.shields.io/badge/cargo%20tree-$dep_count-blue"
    size_img="https://img.shields.io/badge/build%20size-${size_mb}M-green"

    echo "[![Dependencies: ${dep_count}](${deps_img})](${crates_io_url})"
    echo "[![Binary Size: ${size_mb}M](${size_img})](${crates_io_url})"

# Update the blazon-generated badges in README.md using textum
update-readme-badges:
    #!/usr/bin/env bash
    set -euo pipefail
    NEW_CONTENT=$(just blazon-md)
    textum replace \
        "<!-- just: blazon-md -->" \
        $'\n'"${NEW_CONTENT}" \
        --until "<!-- /just: blazon-md -->" \
        README.md
