# What to do next

## Work

A **work item** is either an unfinished feature, or an unfinished component
that has no features. Done and dropped things aren't work.

A work item is **blocked** by every hard need it has that isn't met, both
its own and its component's. It's **ready** when it is wanted and nothing
blocks it. A hard need is one with `on` that isn't `soft` and doesn't
come from a dropped feature.

`next` lists the ready work:

```console
{{#include ../transcripts/next.txt}}
```

`next --all` lists the blocked work too, each item with what blocks it:

```console
{{#include ../transcripts/next-all.txt}}
```

Items are grouped by milestone (the example has none, hence
`no milestone`), then sorted by the band of their area from top to bottom,
then by label.

## Reading one blocked item

Name a feature after `#` to look at just that part of a component:

```console
{{#include ../transcripts/show-one-source.txt}}
```

The `>` marks the feature you asked about. The needs list shows only its
needs and the component's own. The `blocked:` section at the end says the
same thing as `next --all`, for this feature only.

The note explains the work: the text of the agent prompt is built in two
places today. The fix is to build it once, in Rust, and send it in the JSON,
so the page and the terminal read the same text. That's the JSON's
`prompts` feature, and the reason one-source needs it.

`rdeps` on a feature shows only what needs that feature:

```console
{{#include ../transcripts/rdeps-prompts.txt}}
```

## When the blocker lands

Suppose someone finishes the JSON prompts and sets that feature to
`status = "done"`. The JSON component becomes `done`, the need is met, and
the blocked item moves up:

```console
{{#include ../transcripts/prompts-done-next.txt}}
```

Nothing else had to change. You don't move tickets between columns; you
record what's true about the code, and the map works out what that means.
The [roadmap view](page.md#roadmap) draws the same thing as columns: work
that's ready now in the first, and each later column holding work whose
blockers are all in the columns before it.
