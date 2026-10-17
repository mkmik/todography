# Keeping the map with the code

A map is only worth reading if it's true. The way to keep it true is to
change it in the same commit as the code it describes, so it goes through
the same review.

## When you change code

- **A feature lands.** Set its `status = "done"`.
- **You start on something.** Set it to `wip` if that helps others; it's
  optional.
- **You find something missing in another layer.** Add a `[[feature]]` with
  `status = "todo"` to that layer's component, and a need on it from yours,
  in the same change. Record it where it belongs, not where you were
  working.
- **A new piece appears because something needed it.** Add a component
  with `intent = "induced"`, and the needs that pulled it in.
- **A plan changes.** Set features you won't build to `dropped` instead of
  deleting them, so the history of the decision stays readable.
- **Before you commit,** run `todography lint`.

## Reviewing a change to the map

A map change in a pull request is small and reads like prose. Things to
look for:

- Status changes match the code in the same pull request.
- New needs have a `why` that a newcomer would understand.
- New components say why they're on the map: a goal, or the needs that
  induce them.
- Nothing became a goal by accident. `intent = "goal"` widens what's wanted,
  and everything it needs comes with it.

## Referring to the map

In commit messages, code comments and design documents, write labels in
full: `t8y:/exec/mailboxes#attention-asts`. Then `grep -r t8y:` finds every
reference to the map in the repository, and the map's `refs` can point back
at those files.

## Making it hard to forget

- Run `todography lint --strict` in CI; see
  [Checking and publishing in CI](ci.md).
- Put the rules above in the instructions your coding agents read; see
  [Working with coding agents](agents.md).
- Publish the page so people look at it, and notice when it's wrong.
