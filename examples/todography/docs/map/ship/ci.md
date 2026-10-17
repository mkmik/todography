+++
title = "CI"
summary = "fmt, clippy -D warnings and tests on every push and pull request"
aka = ["github actions", "ci.yml"]
kind = "infra"
intent = "goal"
research = "done"
status = "done"
refs = [".github/workflows/ci.yml"]

[[needs]]
on = "/ship/tests"
why = "The suite it runs"
+++
