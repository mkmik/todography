+++
title = "TUI map"
summary = "Tiles as glyphs that fill as work gets done: ○ ◌ ◔ ◑ ◕ ●"
kind = "ui"
intent = "goal"
research = "done"
refs = ["src/tui.rs"]

[[feature]]
id = "glyphs"
title = "A glyph and the title per tile; goals bold, unwanted faint"
status = "done"
needs = ["/model/status", "/model/wanted"]

[[feature]]
id = "links"
title = "↓ marks what the selection needs, ↑ what needs it; t dims the rest"
status = "done"
needs = ["/model/graph#needed-by"]

[[needs]]
on = "/tui/terminal"
why = "Lives in a tab of the TUI"
+++
