+++
title = "Roadmap view"
summary = "Wanted work in columns: ready now, then by depth of blockers"
aka = ["roadmap tab", "columns"]
kind = "ui"
intent = "goal"
research = "done"
refs = ["assets/map.html"]

[[feature]]
id = "columns"
title = "A column holds work whose blockers are all in earlier columns"
status = "done"

[[feature]]
id = "filters"
title = "Filter by milestone and area; outside blockers still count"
status = "done"

[[needs]]
on = "/page/shell"
why = "Lives in a tab of the page"

[[needs]]
on = "/model/work"
why = "The work items and their blockers"
+++
