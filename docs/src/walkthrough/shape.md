# Bands, areas and components

## The config

Every map starts with `todography.toml`. The example's:

```toml
{{#include ../../../examples/todography/docs/map/todography.toml}}
```

**Bands** are the rows of the drawing, from top to bottom. A good set of
bands reads like layers: what people use at the top, what it stands on at
the bottom. The example puts the command line, the page and the terminal UI
in the top band, the model under them, the file format under that, then
shipping, then third-party crates.

**Kinds** say what sort of thing a component is. They're labels for
people; todography only checks that each component uses a kind from this
list. The example has no milestones, so `todography.toml` lists none. The
[format reference](../reference/format.md#milestones) covers them.

## Areas

Each directory under `docs/map/` is an **area**. The example has eight:

```text
docs/map/
├── todography.toml
├── bridges/    links to other tools
├── cli/        the command line
├── crates/     the crates it depends on
├── format/     the map files and their checks
├── model/      what the map works out
├── page/       the interactive page
├── ship/       tests, CI, releases, docs
└── tui/        the terminal UI
```

Each area has an `AREA.md`. Here is the model's:

```markdown
{{#include ../../../examples/todography/docs/map/model/AREA.md}}
```

`band` places the area in a row of the drawing. The `summary` is one line
that shows wherever the area is listed, and the body is shown when you
click the area's name on the page.

## A component

Every other `.md` file in an area is a component. This is the one for the
JSON output, `cli/json.md`:

```markdown
{{#include ../../../examples/todography/docs/map/cli/json.md}}
```

Reading it from the top:

- `title` is the short name the map shows on its tile.
- `summary` is one line saying what it is. Lint warns past 100 characters.
- `aka` lists other names, so search finds it by them.
- `kind` is one of the kinds in `todography.toml`.
- `intent` is why it is on the map. `induced` means it is here because
  something else needs it; the next page explains the three intents.
- `research` says how well its own dependencies are known. `done` means
  they're all on the map.
- `refs` point at files or URLs that say more.
- Each `[[feature]]` is one thing the component does, with its own status.
  The last one isn't done yet, and has a `note`.
- Each `[[needs]]` is a dependency on another component, with a `why`.

The file's path gives the component its **label**: `cli/json.md` is
`t8y:/cli/json`, and its `prompts` feature is `t8y:/cli/json#prompts`.
Inside map files you can drop the `t8y:` and write `/cli/json#prompts`, or
just `#prompts` from inside `cli/json.md` itself. [Labels](../reference/labels.md)
has the full rules.

The `markdown` feature has a `needs` list of its own. A feature's needs say
that this feature, and not the whole component, depends on something. Here,
rendering Markdown is the only part of the JSON output that uses
pulldown-cmark.

Everything after the closing `+++` is the component's description. This
one is a single sentence; it can be as long as you like.
