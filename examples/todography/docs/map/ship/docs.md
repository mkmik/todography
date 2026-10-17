+++
title = "Documentation"
summary = "The book on GitHub Pages: a walkthrough, guides and the full reference"
aka = ["docs", "book", "mdbook", "github pages"]
kind = "infra"
intent = "goal"
research = "done"
refs = ["docs/src/SUMMARY.md", "docs/build.sh", ".github/workflows/docs.yml"]

[[feature]]
id = "reference"
title = "The format, the commands, the JSON, the page and the terminal"
status = "done"

[[feature]]
id = "walkthrough"
title = "A walkthrough of this map, with output from the real commands"
status = "done"
needs = ["/ship/example"]

[[feature]]
id = "site"
title = "Built with mdBook and published to GitHub Pages from main"
status = "done"
+++

The command output in the walkthrough is generated from this map when the
book is built, so it can't drift from what the tool prints.
