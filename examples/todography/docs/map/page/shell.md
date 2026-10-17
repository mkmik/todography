+++
title = "Page shell"
summary = "Header, counts, search, lint badge, tabs and the view in the URL"
aka = ["header", "search", "map.html"]
kind = "ui"
intent = "induced"
research = "done"
refs = ["assets/map.html"]

[[feature]]
id = "search"
title = "Search box over title, aka, label and summary; / focuses it"
status = "done"

[[feature]]
id = "lint-badge"
title = "A badge with the lint errors and warnings"
status = "done"

[[feature]]
id = "url-state"
title = "The view lives after #, so back, forward and links work"
status = "done"

[[feature]]
id = "themes"
title = "Light and dark, laptop and phone"
status = "done"

[[feature]]
id = "favicon"
title = "Inline SVG favicon"
status = "done"
needs = ["/ship/logo"]

[[needs]]
on = "/cli/json"
why = "Reads exactly the fields json writes"
+++
