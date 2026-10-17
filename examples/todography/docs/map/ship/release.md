+++
title = "Release"
summary = "Every merge to main is a minor release: tag, GitHub release, crates.io"
aka = ["crates.io", "publish", "version bump"]
kind = "infra"
intent = "goal"
research = "done"
refs = [".github/workflows/ci.yml"]

[[feature]]
id = "bump"
title = "Bump the minor version, commit and tag on main"
status = "done"

[[feature]]
id = "crates-io"
title = "Publish to crates.io with trusted publishing (OIDC)"
status = "done"

[[feature]]
id = "gh-release"
title = "GitHub release with generated notes"
status = "done"

[[feature]]
id = "resumable"
title = "Each step checks what exists, so a re-run resumes"
status = "done"

[[needs]]
on = "/ship/ci"
why = "Runs only after the test job passes"
+++
