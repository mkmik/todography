+++
title = "t8y labels"
summary = "t8y: URIs naming components and features"
aka = ["t8y:", "label", "URI"]
kind = "core"
intent = "induced"
research = "done"
refs = ["src/label.rs", "docs/format.md"]

[[feature]]
id = "parse"
title = "t8y:/area/component#feature, /area/component, area/component"
status = "done"

[[feature]]
id = "relative"
title = "Short forms resolved against the file's own label (RFC 3986)"
status = "done"

[[feature]]
id = "refuse-paths"
title = "Refuse relative paths that would change meaning on a move"
status = "done"
+++

A label is a URI whose path is the file's path under `docs/map/` without
`.md`. A label that names nothing is an error, so a typo can't quietly cut
a link.
