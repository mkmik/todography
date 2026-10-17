+++
title = "Jira bridge"
summary = "Keep the leaf tasks in Jira and the shape of the project in the map"
aka = ["jira", "issue tracker"]
kind = "bridge"
intent = "aspiration"
research = "deferred"
status = "todo"

[[needs]]
on = "/model/work"
why = "Work items are what would map to issues"

[[needs]]
fuzzy = "Jira's REST API"
why = "Read issue status back into feature status"

[[needs]]
fuzzy = "a way to tie a Jira issue to a feature"
why = "A field in the feature, or a label on the issue"
+++
