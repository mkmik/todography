+++
title = "Loader"
summary = "Finds the map and reads todography.toml, AREA.md and component files"
aka = ["map.rs", "parser", "front matter"]
kind = "core"
intent = "induced"
research = "done"
refs = ["src/map.rs"]

[[feature]]
id = "discover"
title = "Find todography.toml here or in a directory above; --map DIR"
status = "done"

[[feature]]
id = "config"
title = "Bands, milestones and kinds from todography.toml"
status = "done"

[[feature]]
id = "files"
title = "Areas and components from +++ TOML front matter"
status = "done"

[[feature]]
id = "line-numbers"
title = "Parse errors point at the line in the file, not in the front matter"
status = "done"

[[needs]]
on = "/format/label"
why = "Resolves every need and feature reference"

[[needs]]
on = "/crates/toml"
why = "Parses the config and the front matter"

[[needs]]
on = "/crates/serde"
why = "Deserializes into typed structs, refusing unknown fields"
+++

Reads every file into one `Map`, keeping each problem as a diagnostic with
its file and line instead of stopping at the first one.
