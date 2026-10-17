+++
title = "Renovate"
summary = "Dependency and action update pull requests"
aka = ["dependency updates", "renovate.json"]
kind = "infra"
intent = "goal"
research = "done"
status = "done"
refs = ["renovate.json"]

[[needs]]
on = "/ship/ci"
why = "Its pull requests are only safe to merge on a green CI"
+++
