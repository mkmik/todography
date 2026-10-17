# When the map is wrong

The map is useful only if it can be trusted, so `todography lint` checks
all of it. On the example:

```console
{{#include ../transcripts/lint.txt}}
```

`--strict` makes warnings fail too. The example is checked with `--strict`
on every change to the repository.

## A typo

Labels are how components point at each other, so a mistyped label must
not quietly cut a link. Here is the example with one need in
`tui/roadmap.md` mistyped as `/model/wrok`:

```console
{{#include ../transcripts/typo-lint.txt}}
```

Each line names the file, says whether it's an error or a warning, and
says what's wrong. When a label is close to a real one, lint suggests it.
`lint` exits with status 1 when there are errors, so it can stop a commit
or a CI job.

The other commands still run on a map with errors. `show`, `why`,
`rdeps`, `next` and `debt` print a reminder to standard error first.
`build` writes the page anyway and shows the errors in a badge in its
header, so a broken map still renders and says what's wrong with it.

## What else it checks

Errors are things that make the map wrong: a file that doesn't parse, an
unknown field, a missing title or summary, a label that names nothing, a
kind or band that isn't listed, a component that needs itself.

Warnings are things that are probably mistakes: a summary longer than 100
characters, a missing `research` field, an induced component that nothing
needs, an aspiration that something wanted needs (so it isn't really an
aspiration), and a cycle of hard needs between components.

[Lint](../reference/lint.md) lists every message.
