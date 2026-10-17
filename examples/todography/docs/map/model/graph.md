+++
title = "Needs graph"
summary = "Needs and needed-by between components and features, and which are met"
aka = ["needs", "rdeps", "met"]
kind = "core"
intent = "induced"
research = "done"
refs = ["src/compute.rs"]

[[feature]]
id = "needed-by"
title = "The reverse edges: what needs each component and feature"
status = "done"

[[feature]]
id = "met"
title = "Met: a feature when done, a component when partial or done"
status = "done"

[[feature]]
id = "cycles"
title = "Strongly connected components (Tarjan) for the cycle warning"
status = "done"

[[needs]]
on = "/format/loader"
why = "The edges come from [[needs]] and feature needs"
+++
