# Goals and what they pull in

Every component says why it is on the map, with its `intent`:

- **`goal`**: we want it for its own sake.
- **`induced`**: nobody wants it for itself. It's here because something
  else needs it.
- **`aspiration`**: we might want it some day, but haven't chosen it.

In the example, the goals are the things a person uses or relies on
directly: `lint`, the queries, `build`, each view of the page and of the
terminal UI, the agent prompt, CI, releases, Renovate and the docs.
Everything in the model area is induced. Nobody wants "research debt" or
"the needs graph" for themselves; they exist because the commands and
views need them.

## Wanted

todography marks as **wanted** every goal, and everything a wanted
component needs, through any number of steps. A goal needs the model, the
model needs the loader, the loader needs the `toml` crate, so the `toml`
crate is wanted.

In the example, every component is wanted except the four aspirations.
An induced component that isn't wanted would be one that only unchosen
ideas need. That can be fine, but it's worth a look: either an idea has
been chosen and should be a goal, or the component is waiting on a choice
nobody has made.

## Why

For a wanted component that isn't a goal, todography keeps up to three of
the shortest chains of needs from a goal down to it. `todography why`
prints them:

```console
{{#include ../transcripts/why-base64.txt}}
```

Read each line from left to right: the agent-prompt goal needs the terminal
UI's shell, which needs the base64 crate. The other two chains come from
two of the terminal's views. Every chain ends in `tui/terminal`, so the
answer to "why do we depend on base64?" is "the terminal UI". If you
looked at `tui/terminal.md`, you'd find the reason: its `osc52` feature,
which copies to the clipboard over the terminal, needs base64 to encode
what it sends.

The same chains show in the page's Focus view, on every component's card.

## Who needs it

`why` goes from the goals down. `rdeps` goes the other way, from a
component up to what needs it. With `--all` it keeps going, indenting each
step:

```console
{{#include ../transcripts/rdeps-all-graph.txt}}
```

The first five lines need the needs graph directly; some need a particular
feature of it, such as `#cycles`. Each indented block is what needs those,
one more step up. A component can appear on more than one line, once for
each thing it needs, but todography follows it upward only once, from the
step where it first turns up. Together the lines are everything that could
break if the needs graph changed.
