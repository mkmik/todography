# The terminal

`todography tui` shows the Map, Focus and Roadmap views in the terminal.
It reads the map files directly rather than the page's data, so after you
edit a file, press `r` to read them again and see the change.

```console
cargo run -- --map examples/todography tui
```

The terminal can't draw hover lines, so the TUI says the same things
another way.

## Map (`1`)

Each area's components are listed under its name as a glyph and a title.
The glyph is a circle that fills as the work gets done:

| Glyph | Meaning |
| --- | --- |
| `○` | to do |
| `◌` | in progress |
| `◔` `◑` `◕` | partial, with a few, about half, or most of its features done |
| `●` | done |
| `×` | dropped |

Goals are bold; components that aren't wanted are faint and in italics.
`s` cycles what's shown, as the page's filter does. Move with the arrow
keys. Every tile the selection needs is marked `↓` in blue, and every tile
that needs it `↑` in magenta. `t` dims everything else. The panel at the
bottom shows the selection's summary and names both sets. `Enter` opens
it in Focus.

In the example, select **Agent prompt** in the Page area. Five tiles get a
`↓`, one for each component any of its features needs: the page's Focus
and Roadmap views, the model's Work, the TUI shell, and the JSON output its
unfinished feature waits for.

## Focus (`2`)

Needs on the left, the card in the middle, needed-by on the right, the same
nodes in the same order as the page. `←` and `→` move between columns, `↑`
and `↓` pick within one, `Enter` re-centres on the picked node and
`Backspace` goes back along the trail. In the card, `↑` and `↓` pick a
feature, which greys out the needs it doesn't have; `Esc` lets go of it.

## Roadmap (`3`)

The page's columns. `m` cycles the milestone filter and `Enter` opens the
picked work in Focus.

## Copying

`c` copies the agent prompt for the work you've picked, the same text as
the page's copy button. The TUI sends it with the OSC 52 escape sequence,
so it works over ssh. Your terminal has to allow it; in tmux that means
`set-clipboard on`.

The mouse works too: click to pick, click again to open, scroll with the
wheel. Since the TUI holds the mouse, your terminal can't select text, so
the TUI does it for you: drag within a column to select, and releasing
copies it.

[The terminal UI reference](../reference/tui.md) lists every key.
