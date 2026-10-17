# Aspirations and research debt

## Ideas on the map

An **aspiration** is something the project might want but hasn't chosen.
Putting it on the map is cheap, and it records what it would take. The
example has four:

| Component | Idea |
| --- | --- |
| `t8y:/format/tags` | Tags that group components across areas, like epics |
| `t8y:/model/buckets` | Now, soon and later buckets for work |
| `t8y:/page/canvas-layout` | Dragging tiles to place them by hand |
| `t8y:/bridges/jira` | Keeping leaf tasks in Jira and the shape in the map |

Aspirations aren't wanted unless something wanted needs them, so they
don't show up in `next`, and the page draws them dashed and grey. `show`
says so:

```console
{{#include ../transcripts/show-jira.txt}}
```

The Jira bridge has `research = "deferred"`: nobody has worked out what it
would depend on. Two of its needs are **fuzzy**. They name dependencies in
words because they aren't components on the map, and maybe never will be.

## Research debt

A wanted component has **research debt** when its research is deferred or
it has a fuzzy need. That's a wanted piece whose cost nobody knows yet.

The example has none:

```console
{{#include ../transcripts/debt.txt}}
```

Aspirations can carry deferred research and fuzzy needs without debt,
because nobody has chosen them. The moment one is chosen, its unknowns
become debt.

## Choosing one

To choose an aspiration, set its `intent = "goal"`. Here is the Jira bridge
chosen:

```console
{{#include ../transcripts/jira-debt.txt}}
```

The debt list is a to-do list for thinking: look into each fuzzy need, and
either replace it with a label pointing at a component (adding the
component if it's new), or decide it isn't needed. When the research is
done, set `research = "done"`.

Its one hard need, on the model's work items, is met, and fuzzy needs
never block, so the bridge is ready at once:

```console
{{#include ../transcripts/jira-next.txt}}
```

That's on purpose. The map can't know what a fuzzy need stands for, so it
reports it as debt instead of guessing.

## Previewing a choice

Editing a file to try a choice is a commitment you'd have to undo. The
page's [Choose view](page.md#choose) lets you preview it instead: it lists
every aspiration with what choosing it would pull in, and "Preview as
chosen" recomputes wanted, ready and debt in the browser, for every view,
without touching any file.
