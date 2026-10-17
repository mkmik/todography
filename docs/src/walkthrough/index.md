# The example map

The walkthrough uses one map from start to finish: todography's map of
itself. It is in the repository at
[`examples/todography/docs/map`](https://github.com/mkmik/todography/tree/main/examples/todography/docs/map).
It describes the real tool: the file format and its checks, the model that
works things out, the command line, the page, the terminal UI, how the
project ships, and the crates underneath. It also holds a few ideas that
nobody has chosen to build yet.

You can [open its page](../example/index.html) in another tab and follow
along.

## Following along

Clone the repository and run the commands from `examples/todography`, or
pass `--map` from anywhere:

```console
git clone https://github.com/mkmik/todography
cd todography
cargo run -- --map examples/todography next --all
```

Every block of command output in this walkthrough was produced by running
that command on this map. The test suite runs the same commands and fails
if the output changes, so what you read here is what the tool prints.

## The pages

1. [Bands, areas and components](shape.md): how the map is laid out on
   disk and what one component file looks like.
2. [Goals and what they pull in](wanted.md): goals, induced components,
   and why each piece is on the map.
3. [Features and status](status.md): how a component's status comes from
   its features, and when a need counts as met.
4. [What to do next](next.md): ready and blocked work, and what changes when
   a blocker lands.
5. [Aspirations and research debt](aspirations.md): ideas you haven't
   chosen, and what choosing one would cost.
6. [When the map is wrong](lint.md): what `lint` catches and how it says so.
7. [The page](page.md): the four views of the interactive map.
8. [The terminal](terminal.md): the same views without a browser.
