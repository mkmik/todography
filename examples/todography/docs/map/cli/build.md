+++
title = "Build"
summary = "todography build: write the page with the map's JSON baked in"
aka = ["build", "out/map/index.html"]
kind = "core"
intent = "goal"
research = "done"
refs = ["src/json.rs", "assets/map.html"]

[[feature]]
id = "inline"
title = "Replace __MAP_DATA__ in the template, built in with include_str!"
status = "done"

[[feature]]
id = "despite-lint"
title = "Write the page even when lint finds errors"
status = "done"

[[feature]]
id = "out"
title = "--out PATH, else out/map/index.html under the workspace root"
status = "done"

[[needs]]
on = "/cli/json"
why = "The data it bakes in"

[[needs]]
on = "/page/shell"
why = "The template it fills"
+++
