+++
title = "Format"
band = "format"
summary = "The map files: labels, parsing, and the checks on them"
+++

What a map is on disk: `todography.toml`, one `AREA.md` per directory and
one Markdown file per component, with TOML front matter. `src/map.rs` and
`src/label.rs` read it; the format reference in the documentation says
what goes in each file.
