# The map format

A map is a directory of plain files, usually `docs/map/` at the root of a
repository. This page lists everything that can go in them. For a gentler
start, see [Getting started](../getting-started.md); for the reasons behind
the format, see the [walkthrough](../walkthrough/index.md).

## Layout

```text
docs/map/
├── todography.toml        what the whole map shares
├── README.md              optional; skipped
├── exec/                  an area
│   ├── AREA.md            what the area is
│   ├── mailboxes.md       a component
│   └── qio.md             a component
└── net/                   another area
    ├── AREA.md
    ├── ssl.md
    └── ip/                an area inside an area
        ├── AREA.md
        └── tcp.md
```

- `todography.toml` sits at the top of the map directory.
- Every directory below it is an **area**, and must have an `AREA.md`.
  Areas can nest; a nested area's id is its path, such as `net/ip`.
- Every other `.md` file in an area is a **component**. Its label comes
  from its path: `net/ip/tcp.md` is `t8y:/net/ip/tcp`.
- Files named `README.md` are skipped anywhere, so you can explain the map
  to people browsing the repository.
- Files and directories whose names start with `.` are skipped, and so are
  files that don't end in `.md`.
- A component file directly in the map directory, outside any area, is an
  error.

Directory and file names (without `.md`) must be lowercase letters, digits
and `-`, starting with a letter or digit.

## todography.toml

```toml
title = "vaxpunk"

[[band]]
id = "apps"
title = "Applications"

[[band]]
id = "exec"
title = "Executive"

[[milestone]]
id = "m1"
title = "Milestone 1"
summary = "What it delivers"   # optional
refs = ["docs/m1.md"]          # optional

[[kind]]
id = "core"
summary = "Part of the base OS"   # optional
```

| Field | Required | Meaning |
| --- | --- | --- |
| `title` | yes | The map's name, shown in the page's header and tab |
| `[[band]]` | | A row of the drawing. Listed top to bottom |
| `band.id` | yes | Its id, used by areas' `band` field |
| `band.title` | yes | The name the page shows |
| `[[milestone]]` | | A milestone that features and components can belong to. Listed in order |
| `milestone.id` | yes | Its id, used by `milestone` fields and `next --milestone` |
| `milestone.title` | yes | The name `next` and the page show |
| `milestone.summary` | no | One line about it |
| `milestone.refs` | no | Paths or URLs that say more |
| `[[kind]]` | | A kind of component |
| `kind.id` | yes | Its id, used by components' `kind` field |
| `kind.summary` | no | What the kind means |

Ids are lowercase letters, digits and `-`. Band and milestone ids must be
unique among bands and milestones together; kind ids only among kinds, so
a kind can share an id with a band.

No other fields are allowed. An unknown field is an error, so a typo in a
field name is caught instead of ignored.

### Milestones

Milestones are optional. A feature's `milestone` says which milestone it
belongs to; a component's `milestone` applies to the component, and to any
of its features that don't name their own. `todography next` groups work by
milestone, in the order they're listed, and `next --milestone ID` shows one.
The page's roadmap filters by milestone, and starts on the one with id `m1`
if any wanted work is in it.

## Front matter

`AREA.md` and component files start with TOML front matter between two
lines that are exactly `+++`. The first line of the file must be `+++`; the
front matter ends at the next line that is exactly `+++`. Everything after
that is the body, in Markdown. Windows line endings and a leading byte
order mark are accepted.

```markdown
+++
title = "Exec"
+++

The body.
```

Bodies are shown on the page, rendered as Markdown with tables,
strikethrough and task lists.

## AREA.md

```markdown
+++
title = "Executive"
band = "exec"
summary = "Processes, scheduling, system services"
+++

What the area holds, and where it sits in the picture.
```

| Field | Required | Meaning |
| --- | --- | --- |
| `title` | yes | The name the page shows |
| `band` | yes | The band it sits in, from `todography.toml` |
| `summary` | yes | One line. Lint warns past 100 characters |

## A component

```markdown
+++
title = "Mailboxes"
summary = "Message queues between processes, as MBAn: devices"
aka = ["MBX", "$CREMBX"]
kind = "core"
intent = "goal"
research = "done"
milestone = "m1"
refs = ["docs/adr/0019-mailboxes.md", "vms/exec/mbdriver.mar"]

[[feature]]
id = "basic"
title = "$CREMBX, $DELMBX, reads and writes"
status = "done"

[[feature]]
id = "attention-asts"
title = "Read and write attention ASTs"
status = "todo"
milestone = "m1"
note = "Needs the AST delivery rework first"
refs = ["docs/adr/0023-asts.md"]
needs = ["/exec/asts#delivery"]

[[needs]]
on = "/exec/qio"
why = "Its reads and writes are $QIO requests"

[[needs]]
on = "/exec/timers"
soft = true
why = "Read timeouts; reads just wait forever without it"

[[needs]]
fuzzy = "a way to name mailboxes across a cluster"
why = "Cluster-wide mailboxes"
+++

What a mailbox is, in a paragraph or a few. The page shows this when you
open the component.
```

| Field | Required | Meaning |
| --- | --- | --- |
| `title` | yes | The short name on the tile |
| `summary` | yes | One line: what it is. Lint warns past 100 characters |
| `aka` | no | Other names it goes by, for search |
| `kind` | yes | One of the kinds in `todography.toml` |
| `intent` | yes | `goal`, `aspiration` or `induced` |
| `status` | no | `done`, `partial`, `wip`, `todo` or `dropped`. Only for a component without features |
| `research` | should | `done`, `outline` or `deferred`. Missing is a warning, and counts as `deferred` |
| `milestone` | no | A milestone id from `todography.toml` |
| `refs` | no | Repository paths or URLs that say more |
| `[[feature]]` | no | What it does, in the order it grows |
| `[[needs]]` | no | What it depends on |

### intent

Why the component is on the map.

- `goal`: wanted for its own sake.
- `induced`: here because something else needs it. Nobody wants it for
  itself.
- `aspiration`: something the project might want some day, but hasn't
  chosen.

Goals, and everything they need, are [wanted](computed.md#wanted).

### status

For a component without features, its own status. Without a `status`
field, it is `todo`. A component with features gets its status
[from them](computed.md#status), and setting `status` on it is an error.

| Value | Meaning |
| --- | --- |
| `done` | Finished |
| `partial` | Usable, not finished |
| `wip` | Being worked on, not usable yet |
| `todo` | Not started |
| `dropped` | Not going to happen |

### research

How well the component's *own* dependencies are known.

| Value | Meaning |
| --- | --- |
| `done` | Its needs are all on the map |
| `outline` | The big ones are known |
| `deferred` | Not looked into; its needs are guesses |

A wanted component with `research = "deferred"`, or with a fuzzy need, has
[research debt](computed.md#research-debt).

### refs

Free-form strings: repository paths or URLs. The page shows URLs as links
and paths as text. todography doesn't check them.

## Features

Each `[[feature]]` is one thing the component does. List them in the order
the component grows: the first is usually the basic thing that works, the
later ones the gaps.

| Field | Required | Meaning |
| --- | --- | --- |
| `id` | yes | Lowercase letters, digits and `-`; unique in the component |
| `title` | yes | What it is |
| `status` | yes | `done`, `wip`, `todo` or `dropped` |
| `milestone` | no | A milestone id. Without one, the component's milestone applies |
| `note` | no | A line about it: what's left, what's odd |
| `refs` | no | Paths or URLs that say more |
| `needs` | no | A list of labels this feature depends on |

A feature's label is its component's label with `#id`:
`t8y:/exec/mailboxes#attention-asts`.

A feature's `needs` are always hard needs, with no `why`. Use the
component's `[[needs]]` when you want to say why, or to make a need soft or
fuzzy. A feature can need another feature of its own component (`"#basic"`),
but not its own component as a whole, and not itself.

The needs of a `dropped` feature pull nothing in and block nothing.

## Needs

Each `[[needs]]` entry is one dependency of the whole component.

| Field | Required | Meaning |
| --- | --- | --- |
| `on` | one of these | A [label](labels.md): a component, or a feature with `#` |
| `fuzzy` | one of these | Words for a dependency that isn't on the map yet |
| `why` | no | What it's needed for |
| `soft` | no | `true` for a dependency it can live without for a while |

Exactly one of `on` and `fuzzy` must be set.

- A need with `on`, not soft, is a **hard need**. Hard needs make their
  target [wanted](computed.md#wanted), and [block](computed.md#work) work
  until they're [met](computed.md#met).
- A **soft** need is shown, and listed in "needed by", but doesn't make its
  target wanted and never blocks.
- A **fuzzy** need is never met and never blocks. On a wanted component it
  is [research debt](computed.md#research-debt): the prompt to look into it
  and replace it with a label.

A need on a whole component is met once the component is `partial` or
`done`. If you need more than its basics, point at the feature you need.

## Labels

Inside map files, write labels as `/area/component`,
`/area/component#feature`, or `#feature` for a feature of the file's own
component. In prose, comments, commit messages and other documents, write
the full form, `t8y:/area/component`, so `grep t8y:` finds every reference
in the repository. [Labels](labels.md) has the full rules.
