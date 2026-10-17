# The terminal UI

`todography tui` shows the page's Map, Focus and Roadmap views in the
terminal. It works the map out from the files directly, not from the
page's JSON, and reads them again when you press `r`.

It sets no background colours, and uses only foreground colours that read
on light and dark terminal themes alike. A line at the bottom lists the
keys for the current view.

## Everywhere

| Key | Does |
| --- | --- |
| `1`, `2`, `3` | Map, Focus, Roadmap |
| `Tab` | The next view |
| `/` | Search, as the page's search box does |
| `c` | Copy the agent prompt for the picked work |
| `r` | Read the map files again |
| `q`, `Ctrl-C` | Quit |

While searching: type to filter, `↑` and `↓` pick a result, `Enter` opens
it in Focus, `Backspace` deletes, and `Esc` closes the search.

## Map

Each area's components are tiles flowed under the area's name. A tile is a
glyph and the title:

| Glyph | Status |
| --- | --- |
| `○` | todo |
| `◌` | wip |
| `◔` | partial, less than 3/8 of its features done |
| `◑` | partial, from 3/8 up to 3/4 done |
| `◕` | partial, 3/4 or more done |
| `●` | done |
| `×` | dropped |

Goals are bold. Tiles that aren't wanted are faint italics. The selection
is in reverse video. Every tile it needs is marked `↓` in blue, and every
tile that needs it `↑` in magenta. The panel at the bottom has the
selection's summary and the names of both sets.

| Key | Does |
| --- | --- |
| arrows, `h` `j` `k` `l` | Move the selection |
| `PgUp`, `PgDn` | Move ten rows |
| `Enter` | Open the selection in Focus |
| `s` | Cycle what shows: wanted + aspirations, everything, wanted only |
| `t` | Make everything the selection isn't related to faint |

## Focus

Needs on the left, the card in the middle, needed by on the right: the same
nodes, in the same order, as the page.

| Key | Does |
| --- | --- |
| `←` `→`, `h` `l` | Move between the three columns |
| `↑` `↓`, `k` `j` | Pick in a column. In the card, pick a feature, which greys out the needs it doesn't have |
| `Esc` | Let go of the picked feature |
| `Enter` | In a side column, re-centre on the picked node |
| `Backspace` | Go back along the trail |
| `PgUp`, `PgDn` | Scroll the card |

## Roadmap

The page's columns.

| Key | Does |
| --- | --- |
| arrows, `h` `j` `k` `l` | Pick work |
| `PgUp`, `PgDn` | Move ten rows |
| `m` | Cycle the milestone: all, each milestone in turn, no milestone |
| `Enter` | Open the picked work in Focus |

## Mouse

A click works like a tap on the page: the first click on a tile, a node, a
feature, a roadmap item, a search result or a tab picks it, and a second
click on the same thing opens it. A click on the trail goes back to that
point. The wheel scrolls the map and the card, and moves the pick in the
column under the pointer.

While the TUI has the mouse, the terminal can't select text, so the TUI
does it: drag within a column to select, and releasing copies the text.

## Clipboard

`c` and drag-to-copy send the text with the OSC 52 escape sequence. That
works over ssh and inside tmux, but the terminal must allow it. In tmux,
set `set-clipboard on`. The copied prompt is the same text the page's copy
button writes; see [the agent prompt](page.md#the-agent-prompt).
