+++
title = "TUI focus"
summary = "Needs, card, needed-by in three columns, with a trail"
kind = "ui"
intent = "goal"
research = "done"
status = "done"
refs = ["src/tui.rs"]

[[needs]]
on = "/tui/terminal"
why = "Lives in a tab of the TUI"

[[needs]]
on = "/model/why"
why = "The card shows the why chains"

[[needs]]
on = "/model/work"
why = "The card says what blocks it"
+++
