+++
title = "Agent prompt"
summary = "Copy a prompt for a coding agent: the work, its file, its blockers"
aka = ["copy button", "workPrompt"]
kind = "ui"
intent = "goal"
research = "done"
refs = ["assets/map.html", "src/tui.rs"]

[[feature]]
id = "page-copy"
title = "Copy button on wanted work in Focus and Roadmap"
status = "done"
needs = ["/page/focus", "/page/roadmap"]

[[feature]]
id = "blockers"
title = "Blocked work lists every unfinished blocker, deepest first"
status = "done"
needs = ["/model/work"]

[[feature]]
id = "tui-copy"
title = "c in the terminal puts the same prompt on the clipboard"
status = "done"
needs = ["/tui/terminal#osc52"]

[[feature]]
id = "one-source"
title = "One copy of the prompt text instead of two"
status = "todo"
note = "Today the text lives in both src/tui.rs and assets/map.html, and a change has to be made twice"
needs = ["/cli/json#prompts"]
+++

Turns a tile into a task an agent can pick up, with the `show` command for
the rest of the context.
