+++
title = "Queries"
summary = "show, why, rdeps, next and debt, as text"
aka = ["show", "why", "rdeps", "next", "debt"]
kind = "core"
intent = "goal"
research = "done"
refs = ["src/main.rs", "src/query.rs"]

[[feature]]
id = "show"
title = "show LABEL: one component or feature, its needs, blockers, why"
status = "done"
needs = ["/model/why", "/model/work"]

[[feature]]
id = "why"
title = "why LABEL: the chains from the goals down to it"
status = "done"
needs = ["/model/why"]

[[feature]]
id = "rdeps"
title = "rdeps [--all] LABEL: what needs it, transitively with --all"
status = "done"
needs = ["/model/graph#needed-by"]

[[feature]]
id = "next"
title = "next [--all] [--milestone M]: ready work, and blocked work"
status = "done"
needs = ["/model/work"]

[[feature]]
id = "debt"
title = "debt: wanted components not researched yet"
status = "done"
needs = ["/model/debt"]

[[needs]]
on = "/format/label#parse"
why = "Every query takes a label"
+++
