# Mapping an existing project

The first map of a real codebase is the hardest one to write, because
everything is already there. This page is advice for getting a useful map
quickly and letting it fill in over time.

## Start from what people use

List the things a user, an operator or another team would name if you
asked what the project does. Those are your goals. In the example, they're
the commands, the views of the page and the terminal UI, and the things
that keep the project shipping, such as CI and releases.

Don't start from the module tree. Modules become components when something
needs them, and the map is clearer when each component is there for a
reason.

## Draw the bands, then the areas

Bands are layers, top to bottom. Two to five is plenty. A common shape is
what people use, the logic under it, the data or formats under that, the
platform, and third-party pieces at the bottom.

Areas group components that belong together, usually along the lines of
your code's directories or your team's vocabulary. An area can be small.
You can move components between areas later; only their labels change, and
lint tells you every reference to fix.

## Follow the needs down

For each goal, ask what it needs. Add those as `induced` components, with a
`[[needs]]` entry and a `why`. Then ask the same of each of them. Stop when
you reach things you don't need to reason about.

When you aren't sure what something needs, don't guess silently:

- write `research = "deferred"` on it, and
- add a `fuzzy` need for each dependency you suspect but can't name.

Both show up in `todography debt`, which becomes the list of things to look
into. As you learn more, replace fuzzy needs with labels and set `research`
to `outline` or `done`.

## Use features for what's unfinished

A component that's simply done can have `status = "done"` and no features.
Give a component features when parts of it are done and parts aren't, or
when other components need only part of it. List them in the order the
component grows: the basic thing first, the gaps after.

Point needs at features when only a feature is needed. A need on a whole
component is met as soon as the component is partial, which is right for
"needs it to exist" and wrong for "needs it to do X".

## Put ideas on the map

Ideas nobody has chosen go in as `aspiration`, with whatever you know about
what they'd need. They stay out of `next` and out of the wanted count, and
the page's Choose view shows what each would cost. That's a better place
for them than a wiki page that nobody reads.

## Let lint guide you

Run `todography lint` often while you work. Its warnings point at the gaps:

- `induced, but nothing needs it`: either something should need it, or it
  should be a goal, or it doesn't belong.
- `an aspiration, but wanted`: something you want needs it, so it isn't
  optional any more. Make it induced, or a goal.
- `research is missing`: decide how well you know its needs.

Then open the page and look at it. The map view shows at a glance whether
the picture matches what you think the project is.
