# Working with coding agents

The map is plain text in the repository, so coding agents can read it and
keep it up to date the same way people do. todography helps in two ways:
it hands agents well-formed tasks, and it gives them commands to look
things up.

## Handing out work

Every piece of wanted work on the page has a copy button, and `c` in the
terminal UI copies the same text. It's a prompt you can paste into an
agent. Here is the one for the example's blocked item, the single source
for the agent prompt:

```text
{{#include ../transcripts/prompt-one-source.txt}}
```

The prompt names the work and its label, includes the feature's note, and,
because the item is blocked, lists every unfinished blocker in the order
they have to be done. It tells the agent where the map file is and how to
read more with `todography show`, and asks it to mark things done when they
land.

For ready work the prompt is shorter: what to implement, the note or
summary, where the map file is, and the reminder.

## Commands agents can use

These print plain text meant for both people and agents:

| Command | Gives the agent |
| --- | --- |
| `todography show LABEL` | Everything about a component or feature, including what blocks it and why it's wanted |
| `todography why LABEL` | Why a component exists |
| `todography rdeps --all LABEL` | What could break if it changes |
| `todography next --all` | What's ready and what's blocked |
| `todography lint` | Whether its edits to the map are valid |
| `todography json` | Everything, for anything the others don't cover |

## Instructions for your agents

Add something like this to the instructions your agents read, such as
`AGENTS.md` or `CLAUDE.md`:

```markdown
## The map

This repository keeps a map of its components in docs/map/: one Markdown
file per component, with TOML front matter. The format is described at
https://mkmik.github.io/todography/reference/format.html.

When you change code:

- When a feature lands, set its status = "done" in its map file.
- If you find something missing in another component, add a [[feature]]
  with status = "todo" there, and a need on it from the component you're
  working on, in the same change.
- If you add a new piece because something needed it, add a component
  with intent = "induced" and the needs that pulled it in.
- Refer to components in comments and commit messages as
  t8y:/area/component.
- Run `todography lint` before committing, and fix what it reports.
```

Agents are good at these edits, because the rules are local: every change
is to the file of the thing that changed, and lint checks every reference.
