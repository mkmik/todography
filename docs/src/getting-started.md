# Getting started

## Install

todography is a single binary written in Rust. With a Rust toolchain
installed:

```console
cargo install todography
```

To build the latest code from the repository instead:

```console
cargo install --git https://github.com/mkmik/todography
```

## A first map

A map lives in `docs/map/` at the root of your repository. It needs three
kinds of file:

- `docs/map/todography.toml`, which names the map and lists its bands and
  its kinds of component.
- One directory per **area**, with an `AREA.md` that says what the area is.
- One Markdown file per **component**, inside an area's directory.

The map on this page is in the repository as
[`examples/quickstart`](https://github.com/mkmik/todography/tree/main/examples/quickstart),
and the output shown here comes from running todography on it.

Start with the config, `docs/map/todography.toml`:

```toml
{{#include ../../examples/quickstart/docs/map/todography.toml}}
```

Bands are the horizontal layers of the drawing, listed top to bottom.
Kinds say what sort of thing a component is. You choose both; todography
only checks that every area names a band from this list and every component
a kind.

Then an area, `docs/map/core/AREA.md`:

```markdown
{{#include ../../examples/quickstart/docs/map/core/AREA.md}}
```

The lines between the two `+++` markers are TOML. Everything after the
second marker is the body, in Markdown, which the page shows when you open
the area. The directory's name, `core`, is the area's id.

Now two components. The first is a goal, `docs/map/core/sync.md`:

```markdown
{{#include ../../examples/quickstart/docs/map/core/sync.md}}
```

A component is a goal when you want it for its own sake. Its features list
what it does, in the order it grows. The second feature needs a feature of
another component, written as a label: `/core/watch#recursive`.

The second component is that dependency, `docs/map/core/watch.md`:

```markdown
{{#include ../../examples/quickstart/docs/map/core/watch.md}}
```

`intent = "induced"` says nobody wants the watcher for itself. It is on
the map because something else needs it.

## Ask the map

Run `todography` from anywhere inside the repository. It finds
`docs/map/todography.toml` by looking in the current directory and the
ones above it.

```console
{{#include transcripts/quickstart-lint.txt}}
```

`lint` checks every file and every reference between them. A clean map has
no errors and no warnings. Now ask what there is to do:

```console
{{#include transcripts/quickstart-next-all.txt}}
```

Two-way sync can't start until the watcher can watch a whole tree, so the
watcher's feature is the one that's ready. Without `--all`, `next` lists
only the ready work.

`show` prints everything about one component: its facts, its features, its
needs and whether each is met, why it's on the map, and what blocks it.

```console
{{#include transcripts/quickstart-show-sync.txt}}
```

Sync is `partial` because one of its two features is done. `why` explains
a component that isn't a goal by printing the chains of needs that lead
down to it:

```console
{{#include transcripts/quickstart-why-watch.txt}}
```

## See it

```console
$ todography build
out/map/index.html
```

That writes one self-contained HTML file under the repository root. Open it
in a browser for the map, a focus view of each component, a roadmap, and a
list of options you haven't chosen yet. `todography tui` shows the map,
focus and roadmap views in the terminal.

## Next

The [walkthrough](walkthrough/index.md) goes through every idea on a
bigger map, todography's own. The [format reference](reference/format.md)
lists every field, and [Mapping an existing project](guides/first-map.md)
has advice for drawing the first map of a real codebase.
