+++
title = "JSON"
summary = "todography json: the whole map and everything worked out, as JSON"
aka = ["json", "json.rs"]
kind = "core"
intent = "induced"
research = "done"
refs = ["src/json.rs"]

[[feature]]
id = "export"
title = "Components, features, needs, status, wanted, why, work, debt, lint"
status = "done"

[[feature]]
id = "markdown"
title = "Bodies and AREA.md rendered to HTML"
status = "done"
needs = ["/crates/pulldown-cmark"]

[[feature]]
id = "reproducible"
title = "generated is SOURCE_DATE_EPOCH when set"
status = "done"

[[feature]]
id = "prompts"
title = "Each work item carries its agent prompt, written once in Rust"
status = "todo"
note = "Then the page and the terminal read the same text instead of each building it"

[[needs]]
on = "/model/work"
why = "Exports work items with their blockers"

[[needs]]
on = "/model/why"
why = "Exports the why paths"

[[needs]]
on = "/model/debt"
why = "Exports the research debt"

[[needs]]
on = "/format/lint"
why = "Lint findings travel with the data"

[[needs]]
on = "/crates/serde"
why = "serde_json writes it"
+++

The page reads exactly the fields this writes, so the two change together.
