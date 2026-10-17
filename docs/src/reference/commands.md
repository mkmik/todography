# Commands

```console
{{#include ../transcripts/help.txt}}
```

## Finding the map

Without `--map`, todography looks in the current directory and then each
directory above it, and uses the first one that has either
`todography.toml` (so you can be inside the map) or
`docs/map/todography.toml` (so you can be anywhere in the repository).

`--map DIR` names the map directory itself, or a workspace root that has
`docs/map/todography.toml` below it.

The **workspace root** is the directory above `docs/` when the map is at
`docs/map`, and otherwise the map directory's parent. File names in output
are relative to it, and `build` writes under it by default.

## Options

Options can go before or after the command, and `--name value` can also be
written `--name=value`.

| Option | Commands | Meaning |
| --- | --- | --- |
| `--map DIR` | all | Where the map is |
| `--strict` | `lint` | Fail on warnings too |
| `--all` | `rdeps`, `next` | Go further: see each command |
| `--milestone M` | `next` | Only work in milestone `M` |
| `--out PATH`, `-o PATH` | `build` | Where to write the page |
| `-h`, `--help` | | Print the usage above |

An option a command doesn't take is an error.

## Exit status

| Status | When |
| --- | --- |
| 0 | Success |
| 1 | `lint` found errors, or warnings with `--strict` |
| 2 | No command, an unknown command or option, a missing argument, no map found, a label or milestone that doesn't exist, or a file that can't be written |

`todography` with no command prints the usage and exits with 2;
`todography help` prints it and exits with 0. Errors go to standard error,
prefixed `todography:`.

## Lint errors and other commands

`show`, `why`, `rdeps`, `next` and `debt` work on a map with lint errors,
using whatever loaded. They first print `todography: the map has N lint
errors; see todography lint` to standard error. `json` and `build` don't
print that; the errors travel in their output instead.

## lint

```console
todography lint [--strict]
```

Checks the map and prints one line per problem, then a summary line. See
[Lint](lint.md) for every message.

## show

```console
todography show LABEL
```

Prints one component:

- its label and title, summary and other names;
- a line of facts: kind, intent, status, research, milestone, wanted or
  not, and `research debt` if it has some;
- its refs, and its file;
- `features:` each feature's status, id and title, with its milestone in
  parentheses and its note on the next line;
- `needs:` each need with its state in brackets (`met`, `unmet`,
  `unmet, soft`, `unmet, dropped feature`, `fuzzy` or `unresolved`) and its
  `why`. A feature's needs start with `#feature ->`;
- `needed by:` each component that needs it, with `#feature` for the
  feature the need comes from, `-> #feature` for the feature it points at,
  `[soft]`, and the `why`;
- `why:` its why paths, or `a goal`, or `not wanted: no goal needs it`;
- `blocked:` each of its work items that is blocked, and by what.

With `#feature` in the label, the feature is marked with `>`. The needs
are limited to that feature's and the component's own, needed by to needs
on that feature or on the whole component, and blocked to that feature.

## why

```console
todography why LABEL
```

Prints the component's label and title, then its why paths, one per line,
goal first. For a goal it prints `a goal`; for a component that isn't
wanted, `not wanted: no goal needs it`. A `#feature` in the label is
accepted and ignored: why paths are per component.

## rdeps

```console
todography rdeps [--all] LABEL
```

Prints what needs the component, one line per need:

```text
t8y:/a/user#feature -> t8y:/a/target#feature  [soft]
```

With a `#feature` in the label, only needs on that feature. With `--all`,
it then prints what needs each of those, indented one more step, and so on.
Each component is followed upward once, from where it is first found. A
feature that needs a sibling feature of its own component is shown only
when you ask about that feature. With nothing to print, it says `nothing
needs it`.

## next

```console
todography next [--all] [--milestone M]
```

Prints the ready work: wanted features, and wanted components without
features, that aren't done and aren't blocked. With `--all`, it prints the
blocked wanted work too, each item followed by a `blocked by` line.

Work is grouped under a heading per milestone, `id: title`, or `no
milestone`, and ordered as described in
[What the map works out](computed.md#order-of-next). The last line counts
the work: `N ready`, or with `--all`, `N ready, M blocked`.

`--milestone M` keeps only work in milestone `M`. A milestone that isn't in
`todography.toml` is an error.

## debt

```console
todography debt
```

Prints each wanted component with research debt, with its research value
and its fuzzy needs and their `why`, then `research debt: N`.

## json

```console
todography json
```

Prints the whole map, and everything worked out from it, as JSON. See
[JSON](json.md).

## build

```console
todography build [--out PATH]
```

Writes the [page](page.md): one self-contained HTML file with the map's
JSON inside it. The default is `out/map/index.html` under the workspace
root. Directories on the way are created. It prints the path it wrote,
relative to the current directory when it's below it.

The page is written even when lint finds errors; it shows them in a badge.
Set `SOURCE_DATE_EPOCH` to fix the build time it records, for reproducible
builds.

## tui

```console
todography tui
```

Opens the [terminal UI](tui.md). It reads the map files itself and can read
them again with `r`.
