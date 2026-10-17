+++
title = "Tags"
summary = "Cross-cutting tags that group components across areas, like epics"
aka = ["epics", "cross-cutting"]
kind = "core"
intent = "aspiration"
research = "deferred"
status = "todo"

[[needs]]
on = "/format/loader"
why = "A new field in the front matter, and a list of tags in todography.toml"

[[needs]]
on = "/format/lint"
why = "An unknown tag should be an error, like an unknown kind"

[[needs]]
fuzzy = "a tag filter in the page and the terminal"
why = "Tags are only worth having if you can look at one"
+++

Areas cut the project by component; tags would cut it by concern
(security, performance, a feature that spans layers).
