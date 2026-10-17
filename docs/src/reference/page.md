# The page

`todography build` writes one HTML file. The [walkthrough](../walkthrough/page.md)
describes each view on the example map; this page lists the details.

## What's in the file

The template is built into the binary. `build` puts the map's
[JSON](json.md) into it and writes the result. The file loads no scripts,
fonts or styles from other servers. The one exception is images: a
Markdown image in a component's or an area's body becomes an `<img>`, and
the browser fetches it from its URL. You can open
it from disk, send it as an attachment, or serve it from any static host.
In the JSON inside the page, `</` is written `<\/` and `<!--` as
`\u003c!--`, so nothing in a map file can end the script early.

The page works in light and dark, on a laptop and on a phone. The tab's
title is the map's title followed by `map`. The footer says when the page
was built, and has a theme selector: system, light or dark.

## Header

- The map's title, and when the page was built.
- Counts: components, wanted, done, partial, and work ready now.
- A search box. It matches title, other names, label and summary. `/`
  focuses it from anywhere on the page; `↑` and `↓` move through the
  results, `Enter` opens one in Focus, and `Esc` clears it.
- A badge with the number of lint errors and warnings, when there are any.
  It opens the list.
- Tabs for the four views.

## Addresses

The current view lives in the address after `#`, so the back and forward
buttons work and you can link to a view. The address holds the view, the
focused component, the map's filter and the roadmap's filters. The Choose
view's filter text and checkboxes aren't in it, and previews live in local
storage.

| Address | Shows |
| --- | --- |
| `#view=map` | The map, showing wanted components and aspirations |
| `#view=map&show=wanted` | The map, wanted components only |
| `#view=map&show=all` | The map, everything |
| `#focus=t8y:/exec/mailboxes` | One component in Focus |
| `#focus=t8y:/rtl/crtl%23stdio` | One component in Focus, with a feature picked |
| `#view=roadmap` | The roadmap, with its default milestone |
| `#view=roadmap&milestone=m0&area=net` | The roadmap filtered to milestone `m0` and area `net` |
| `#view=roadmap&milestone=all` | Every milestone |
| `#view=roadmap&milestone=none` | Work with no milestone |
| `#view=choose` | The choose view |

A label after `focus=` can also be written without `t8y:` or the leading
`/`. A `#` inside it must be written `%23`.

## Map

- Bands top to bottom; each band's areas as boxes; each area's components
  as tiles showing their titles.
- A tile's left edge and fill show its status. A partial tile has a thin
  bar for how much is done. Goals are bold. Wanted tiles are solid; tiles
  that aren't wanted are dashed and grey.
- **Wanted only**, **Wanted + aspirations** (the default) and
  **Everything** choose what shows. In the default, induced components that
  nothing wanted needs are hidden, and an aspiration whose hard needs reach
  hidden components shows `+N`.
- Hovering a tile, or a first tap on a phone, outlines what it needs and
  what needs it, draws lines to them, dims the rest, and shows its summary
  at the bottom. A click, or a second tap, opens Focus.
- Clicking an area's name shows its `AREA.md`.

## Focus

- One component as a card: title, label, other names, summary, chips (kind,
  intent, wanted, status, research, milestone, research debt), progress,
  whether it's ready or what blocks it, its features as a checklist (each
  with status, milestone, note, refs, its own needs and its blockers), the
  why chains, its description, its refs (URLs as links, paths as text) and
  its fuzzy needs.
- On the left, what it needs: one node per component, with each need's
  `why`, the feature it's for, the feature it points at, and whether it's
  met. Soft needs are dashed. Needs on labels the map doesn't have, and
  fuzzy needs, are grey.
- On the right, what needs it. Curves join both sides to the card; on a
  narrow screen they stack without curves.
- Clicking a feature's row keeps lit only the needs it has (its own and the
  whole component's) and points their curves at its row. Clicking it again
  shows them all.
- Clicking a neighbour re-centres on it, and a trail shows where you've
  been.

## Roadmap

- Wanted work in columns: see [Roadmap columns](computed.md#roadmap-columns).
- Filters for milestone (any milestone, all, or no milestone) and area.
  Without a milestone in the address, it starts on `m1` when some wanted
  work is in a milestone with that id, and on all milestones otherwise.
- Columns count blockers outside the filter too, and blockers that are
  ready now are marked.
- Clicking an item opens it in Focus.

## Choose

- The aspirations nothing wanted needs, by band and area. Each lists what
  choosing it would pull in: the components its hard needs reach that
  aren't wanted today (through features that aren't dropped). It counts the
  features, fuzzy needs and components with `research = "deferred"` in
  that set, the aspiration itself included.
- A filter that stays at the top while the list scrolls. Its words match
  the title, label, summary, other names and area; `Esc` clears it.
  **Previewed only** keeps the previewed aspirations. Another checkbox also
  lists the aspirations that something wanted already needs.
- **Preview as chosen** treats an aspiration as a goal on this page only.
  Wanted, ready, research debt and the why chains are worked out again in
  the browser, and the map, roadmap, focus and counts follow. Previews are
  saved in the browser's local storage when it can be used.
- Below the list, the research debt: wanted components not researched yet,
  with their fuzzy needs.

## The agent prompt

Wanted work, in Focus and in the roadmap, has a copy button. It puts a
prompt for a coding agent on the clipboard with:

- what to implement: the work's title, its component and its label;
- the feature's note, or the component's summary for a component without
  features;
- for blocked work, every unfinished blocker, found by following blockers
  all the way down, listed so each comes after what it needs. Each one has
  its title, label and map file, and `ready now` if nothing blocks it. The
  prompt asks the agent to build all of it, or to say so if that's too much
  for one go and propose which dependencies to take first;
- where its map file is, and the `todography show` command for the rest.
  The prompt writes the file as `docs/map/area/component.md`, so for a map
  kept somewhere else the agent should go by the `show` output;
- a reminder to set `status = "done"` when a piece lands.

[Working with coding agents](../guides/agents.md) shows a full prompt.

## Local storage

The page keeps two things in the browser's local storage, when it can:

| Key | Holds |
| --- | --- |
| `todography.theme` | The theme chosen in the footer |
| `todography.preview` | The labels of aspirations previewed as chosen |

Nothing is sent anywhere. Without local storage the page works the same,
and forgets these on reload.
