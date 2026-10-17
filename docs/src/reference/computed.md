# What the map works out

Everything on this page is derived from the files every time todography
runs. None of it is stored, so none of it can go stale. The command line,
the JSON, the page and the terminal UI all use the same results.

## Status

A component **without features** has the status in its `status` field, or
`todo` if there is none.

A component **with features** gets its status from them. Dropped features
don't count:

| Features that aren't dropped | Component status |
| --- | --- |
| none (all dropped) | `dropped` |
| all `done` | `done` |
| at least one `done` | `partial` |
| none `done`, at least one `wip` | `wip` |
| otherwise | `todo` |

## Progress

The page draws a progress bar on partial tiles and the JSON has a
`progress` field. For a component with features, it is the number of done
features out of the features that aren't dropped. For one without features,
it is 1 of 1 when done, 0 of 1 otherwise, and 0 of 0 when dropped.

## Hard needs

A need is **hard** when it has `on`, isn't `soft`, and doesn't come from a
dropped feature. Only hard needs make things wanted and block work.

## Met

| Need | Met when |
| --- | --- |
| on a feature (`/a/b#f`) | that feature is `done`. A need on a dropped feature is never met |
| on a whole component (`/a/b`) | that component is `partial` or `done` |
| fuzzy | never |
| on a label that doesn't resolve | never (and lint reports it) |

A need on a whole component is met as soon as it is usable. When you need
more than its basics, point at the feature you need.

## Needed by

The reverse of needs: for each component, which components (and which of
their features) need it, and which of its features they point at. Every
need except those of dropped features is listed, soft ones included.
`rdeps`, `show` and the page's Focus view show it.

## Wanted

1. Every goal is wanted.
2. Whatever a wanted component hard-needs is wanted.
3. Repeat until nothing changes.

Soft needs, fuzzy needs and the needs of dropped features don't make
anything wanted. An aspiration nothing wanted needs is not wanted, and
neither are the induced components only it needs. If something wanted does
need an aspiration, the aspiration is wanted, and lint warns that it should
be a goal or induced.

## Why paths

For each wanted component that isn't a goal, todography keeps up to three
**why paths**: the shortest chains of hard needs from a goal down to it,
goal first. When several chains have the same length, the order between
them is stable but otherwise arbitrary. Goals have no why paths; they're
wanted for themselves.

## Research debt

A component has research debt when it is wanted and either its `research`
is `deferred` (or missing) or it has a fuzzy need. Components that aren't
wanted never have debt, whatever their research says.

## Work

A **work item** is:

- each feature that isn't `done` or `dropped`, and
- each component without features whose status isn't `done` or `dropped`.

A work item's milestone is its feature's milestone, or else its
component's.

A work item is **blocked by** every unmet hard need it has:

- for a feature, the feature's own needs, and the needs of its component;
- for a component without features, the component's needs.

It is **ready** when it is wanted and nothing blocks it. Work that isn't
wanted is never ready. `next` leaves it out, but the JSON still lists it,
with its blockers, so the page can preview choosing an aspiration.

## Order of `next`

`next` lists wanted work sorted by:

1. milestone, in the order `todography.toml` lists them, then work with no
   milestone;
2. the band of its component's area, top to bottom;
3. label.

## Roadmap columns

The page and the terminal UI place wanted work in columns:

- The first column, "Ready now", holds work nothing blocks.
- Each later column holds work whose blockers are all in earlier columns.

Put another way, an item sits one column after its furthest blocker. A
blocker's own column is found like this:

- a feature, or a component without features, that is a work item: the
  column of that work item;
- a component with features: the column of whichever of its unfinished
  features comes earliest in the roadmap, since one done feature makes it
  usable;
- anything else: the first column. This covers blockers that can never be
  met, such as a need on a dropped feature, which makes the roadmap look
  more hopeful than `next --all`. Lint catches unresolved labels; a need on
  a dropped feature is worth fixing by hand.

Cycles of blockers are cut where the walk meets itself, so every item gets
a column.
