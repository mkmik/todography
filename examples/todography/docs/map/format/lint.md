+++
title = "Lint"
summary = "todography lint: errors and warnings, for CI and before a commit"
aka = ["lint", "check"]
kind = "core"
intent = "goal"
research = "done"
refs = ["src/map.rs", "src/compute.rs", "src/query.rs"]

[[feature]]
id = "errors"
title = "Bad files, unknown fields, dangling labels, missing fields"
status = "done"

[[feature]]
id = "warnings"
title = "Long summaries, missing research, unneeded induced components"
status = "done"

[[feature]]
id = "cycles"
title = "Cycles of hard needs between components"
status = "done"
needs = ["/model/graph#cycles"]

[[feature]]
id = "strict"
title = "--strict: exit 1 on warnings too"
status = "done"

[[needs]]
on = "/format/loader"
why = "Most errors are found while reading"
+++

Exit 1 on errors. The page and the JSON carry the same findings, so a
broken map still renders and says what's wrong.
