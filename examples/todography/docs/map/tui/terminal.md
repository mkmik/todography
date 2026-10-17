+++
title = "TUI shell"
summary = "Tabs, keys, mouse, search, selection, clipboard and reload"
aka = ["tui", "tui.rs"]
kind = "ui"
intent = "induced"
research = "done"
refs = ["src/tui.rs"]

[[feature]]
id = "keys"
title = "1 2 3 for views, / to search, arrows to move, enter to open"
status = "done"

[[feature]]
id = "mouse"
title = "A click picks, a second click opens; the wheel scrolls"
status = "done"

[[feature]]
id = "select"
title = "Drag to select within a column; letting go copies"
status = "done"

[[feature]]
id = "osc52"
title = "Clipboard over OSC 52, so it works over ssh and in tmux"
status = "done"
needs = ["/crates/base64"]

[[feature]]
id = "reload"
title = "r reads the map files again"
status = "done"

[[feature]]
id = "themes"
title = "No background colours; foregrounds that read on light and dark"
status = "done"

[[needs]]
on = "/crates/ratatui"
why = "Draws the screen and reads keys and mouse"

[[needs]]
on = "/format/loader"
why = "Reads the map itself rather than the JSON"
+++
