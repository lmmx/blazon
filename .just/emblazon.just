# Dummy stand-in for what the software would do
blazon-md:
    #!/usr/bin/env bash
    set -euo pipefail

    crates_io_url='https://crates.io/crates/blazon'

    # Data - later will compute these dynamically
    cargo_tree_count="123"
    bin_size="100MB"

    pre='[!['
    mid="]("
    suf=")]($crates_io_url)"

    # deps_img='https://img.shields.io/badge/crates.io-blazon-blue'
    # size_img='https://img.shields.io/badge/crates.io-blazon-blue'
    deps_img="https://img.shields.io/badge/cargo%20tree-$cargo_tree_count-blue"
    size_img="https://img.shields.io/badge/build%20size-$bin_size-blue"

    echo "${pre}Dependencies: ${cargo_tree_count}${mid}${deps_img}${suf}"
    echo "${pre}Binary Size: ${bin_size}${mid}${size_img}${suf}"

# Update the blazon-generated badges in README.md using textum
update-readme-conf:
    #!/usr/bin/env bash
    set -euo pipefail

    # Get the new blazon content (trailing newlines will be stripped by bash)
    NEW_CONTENT=$(just blazon-md)

    # Create patch JSON using jq to build it properly
    # Add newlines in jq where they won't be stripped
    jq -n \
      --arg file "README.md" \
      --arg content "${NEW_CONTENT}" \
      '[{
        file: $file,
        snippet: {
          Between: {
            start: {
              target: {
                Literal: "<!-- just: blazon-md -->"
              },
              mode: "Exclude"
            },
            end: {
              target: {
                Literal: "<!-- /just: blazon-md -->"
              },
              mode: "Exclude"
            }
          }
        },
        replacement: ("\n" + $content + "")
      }]' | textum
