# Features and status

## Status comes from features

A component with features doesn't have a status field. Its status is
worked out from its features, ignoring any that are `dropped`:

- all done: `done`
- some done: `partial`
- none done, but some `wip`: `wip`
- otherwise: `todo`

If every feature is dropped, the component is `dropped` too.

The agent prompt has four features. Three are done and one isn't, so it's
`partial`:

```console
{{#include ../transcripts/show-agent-prompt.txt}}
```

The fourth line of the output collects the facts: kind, intent, status,
research, and whether it's wanted. The `>` in the feature list is absent
here; it appears when you `show` a single feature, as on the next page.

A component without features uses its own `status` field instead. Most of
the model area is like that: `model/wanted.md` says `status = "done"` and
lists no features. That's fine for small pieces where a feature list would
only repeat the title. Setting `status` on a component that has features is
a lint error, because the two could disagree.

## When a need is met

A need on a **feature** is met when that feature is done.

A need on a **whole component** is met once the component is usable: when
it is `partial` or `done`. The idea is that one finished feature makes a
component usable, and if you need more than its basics, you should point
at the feature you need.

The JSON output shows this. It has a feature that isn't done, so it's
`partial`, yet everything that needs the whole component is satisfied:

```console
{{#include ../transcripts/show-json.txt}}
```

`page/shell` needs `t8y:/cli/json`, the whole component, and that need is
met. `page/agent-prompt#one-source` needs `#prompts` specifically, and
that one isn't. The `needed by` section shows both, and the second has
`-> #prompts` to say which feature it points at.

## Soft and fuzzy needs

Two more kinds of need don't appear in the example's `cli/json`:

- A **soft** need (`soft = true`) is one the component can live without for
  a while. It never blocks anything and doesn't make its target wanted.
  The example has none.
- A **fuzzy** need (`fuzzy = "..."`) names a dependency in words because it
  isn't on the map yet. It's never met and never blocks. The aspirations in
  the example use them; see [Aspirations and research debt](aspirations.md).
