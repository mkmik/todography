+++
title = "Status"
summary = "A component's status from its features, or from its own field"
aka = ["done", "partial", "wip", "todo", "dropped"]
kind = "core"
intent = "induced"
research = "done"
status = "done"
refs = ["src/compute.rs"]

[[needs]]
on = "/format/loader"
why = "Reads the feature and component status fields"
+++

All features done is `done`, some done is `partial`, any wip is `wip`,
else `todo`. Dropped features don't count; all dropped drops the component.
