# The page

`todography build` writes the whole map into one HTML file. It has no
scripts, fonts or styles from anywhere else, so you can open it from disk,
attach it to a message, or publish it as a static page. It works on a
phone, and follows your system's light or dark setting unless you pick one
in its footer.

The example's page is published with this book:
**[open the example map](../example/index.html)**.

The header shows the map's title, when it was built, and counts: how many
components, how many are wanted, done and partial, and how much work is
ready now. Next to them is a search box (press `/` to jump to it) that
finds components by title, other names, label or summary. If lint found
anything, a badge in the header says how many errors and warnings, and
opens the list.

There are four views. Each one keeps its state in the address after `#`,
so the back button works and you can link to exactly what you're looking
at.

## Map

[Open the map view](../example/index.html#view=map)

Bands run top to bottom, each band's areas are boxes, and each component is
a tile showing its title. The tile's left edge and fill show its status,
and a partial component has a thin bar showing how much is done. Goals are
in bold. Wanted tiles are solid; tiles that aren't wanted are dashed and
grey.

Above the map you choose what to show:

- **Wanted only** hides everything that isn't wanted.
- **Wanted + aspirations**, the default, shows the aspirations too, but
  hides induced components that only unchosen ideas need. An aspiration
  with such hidden needs shows `+N`.
- **Everything** shows every component.

Point at a tile to outline what it needs and what needs it, with lines to
each, and dim the rest. Its summary shows at the bottom. On a phone, the
first tap does this. Click (or tap again) to open the component in Focus.
Click an area's name to read its `AREA.md`.

## Focus

[Open the agent prompt in focus](../example/index.html#focus=t8y:/page/agent-prompt)

One component as a card in the middle, with what it needs on the left and
what needs it on the right, joined by curves. The card holds everything in
the component's file: its summary, other names, kind, intent, status,
research, milestone, progress, its features as a checklist, its
description and its references. It also holds what the map works out:
whether it's wanted, whether it's ready or what blocks it, and the why
chains from the goals.

On the left, each neighbour is one node, with the `why` of each need, which
feature the need comes from and which feature it points at, and whether
it's met. Soft needs are dashed; fuzzy needs and needs on missing
components are grey.

Click a feature in the checklist to light up only the needs that feature
has (its own, and the whole component's), with the curves pointing at its
row. Click it again to see all of them. Click a neighbour to re-centre on
it; a trail at the top shows where you've been.

Wanted work has a copy button. It puts a prompt on the clipboard that you
can give a coding agent: what to build, where its map file is, and, for
blocked work, every unfinished blocker in the order to do them.
[Working with coding agents](../guides/agents.md) shows the prompt for the
example's blocked item.

On a narrow screen the three columns stack and the curves go away.

## Roadmap

[Open the roadmap](../example/index.html#view=roadmap)

All wanted work in columns. The first column, "Ready now", is the work
nothing blocks. Each later column holds work whose blockers are all in the
columns before it. In the example that's two columns: the JSON prompts,
then the single source for the agent prompt.

You can filter by milestone and by area. Blockers outside the filter still
count when placing work in columns, and blockers that are ready now are
marked. When some work is in a milestone with the id `m1`, the roadmap
starts filtered to it.

## Choose

[Open the choose view](../example/index.html#view=choose)

Every aspiration that nothing wanted needs, grouped by band and area. Each
one lists what choosing it would pull in: the components its hard needs
reach that aren't wanted today, and how many features, fuzzy needs and
unresearched components those hold. A filter at the top matches titles,
labels, summaries, other names and areas.

**Preview as chosen** treats an aspiration as a goal on this page only.
Wanted, ready, research debt and the why chains are worked out again in
the browser, and the map, roadmap, focus and counts all follow. The preview
is kept in your browser's local storage when it can be. Choosing for real
means setting `intent = "goal"` in the file.

Below the list is the research debt: wanted components that aren't
researched yet, with their fuzzy needs.
