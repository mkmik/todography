# Contributing

The code is at [github.com/mkmik/todography](https://github.com/mkmik/todography).
Issues and pull requests are welcome.

## Layout

| Path | What |
| --- | --- |
| `src/main.rs` | The command line |
| `src/map.rs` | Reading the map files, and the checks that need only one file |
| `src/label.rs` | Parsing `t8y:` labels |
| `src/compute.rs` | Status, met, wanted, why paths, debt, work, and the checks that need the whole graph |
| `src/query.rs` | The text that `show`, `why`, `rdeps`, `next`, `debt` and `lint` print |
| `src/json.rs` | The JSON, and filling the page template |
| `src/tui.rs` | The terminal UI |
| `assets/map.html` | The page template, built into the binary |
| `tests/fixture.rs` | Tests on `tests/fixtures/map`, a small map full of deliberate mistakes |
| `tests/example.rs` | Tests on the example maps, and the transcripts the book includes |
| `examples/todography` | todography's map of itself, which the walkthrough uses |
| `examples/quickstart` | The map on the Getting started page |
| `docs/` | This book |

## Tests

```console
cargo test
```

The fixture map in `tests/fixtures/map` is full of mistakes on purpose.
Each one should come out as exactly one lint line, and the tests check
that.

The example maps must lint clean under `--strict`. `tests/example.rs` also
pins every fact the walkthrough states about the example, and compares the
command output in `docs/src/transcripts/` with what the tool prints now.
When you change the tool's output or the example on purpose, rewrite the
transcripts:

```console
TODOGRAPHY_BLESS=1 cargo test --test example
```

Then read the walkthrough pages that include them, and fix any sentence
that now says the old thing.

CI also runs `cargo fmt --check` and `cargo clippy --all-targets -- -D
warnings`.

## Things that change together

- The JSON in `src/json.rs` and the page in `assets/map.html`: the page
  reads exactly the fields the JSON writes.
- The page template is built in with `include_str!`, so rebuild after
  editing it.
- The agent prompt's text is written twice, in `src/tui.rs` and in
  `assets/map.html`. Change both. (The example map has this as its one
  piece of todo work.)

## The book

The book is built with [mdBook](https://rust-lang.github.io/mdBook/)
0.4. To build it with the example's page next to it, as it's published:

```console
docs/build.sh
```

That writes `docs/book/`. Open `docs/book/index.html`. While writing,
`mdbook serve docs` rebuilds on every save, without the example page.

## Releases

Every merge to `main` is a release. CI bumps the minor version, tags it,
publishes the crate to crates.io and creates a GitHub release with
generated notes, after the tests pass. The docs are published to GitHub
Pages from `main` too.
