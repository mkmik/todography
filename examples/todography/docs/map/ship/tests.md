+++
title = "Tests"
summary = "cargo test: a fixture map full of mistakes, and the example map"
aka = ["fixture", "cargo test"]
kind = "infra"
intent = "induced"
research = "done"
refs = ["tests/fixture.rs", "tests/example.rs"]

[[feature]]
id = "fixture"
title = "A small map whose mistakes are on purpose, one lint line each"
status = "done"
refs = ["tests/fixtures/map"]

[[feature]]
id = "example"
title = "The example map lints clean and matches the walkthrough"
status = "done"
needs = ["/ship/example"]
+++
