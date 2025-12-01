#!/bin/sh

set -eu

mkdir -p public/assets

for f in fonts-work/*.woff2; do
    base="$(basename "$f")"
    pyftsubset "$f" \
        --unicodes="U+0020-007F,U+00A0-00FF,U+20AC,U+20BC,U+2010,U+2013,U+2014,U+2018,U+2019,U+201A,U+201C,U+201D,U+201E,U+2039,U+203A,U+2026,U+2022" \
        --output-file="public/assets/$base" \
        --flavor=woff2 \
        --ignore-missing-glyphs \
        --layout-features=ss01,ss02,smcp,c2sc
done