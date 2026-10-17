+++
title = "Work"
summary = "Unfinished features and components: ready, or blocked and by what"
aka = ["next", "ready", "blocked", "blockers"]
kind = "core"
intent = "induced"
research = "done"
status = "done"
refs = ["src/compute.rs", "src/query.rs"]

[[needs]]
on = "/model/status"
why = "Done and dropped work isn't work"

[[needs]]
on = "/model/graph#met"
why = "Blocked means an unmet hard need, its own or its component's"

[[needs]]
on = "/model/wanted"
why = "Ready means wanted and unblocked"
+++
