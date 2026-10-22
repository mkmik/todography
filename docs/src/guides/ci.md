# Checking and publishing in CI

## Lint on every change

`todography lint` exits with status 1 when the map has errors, and
`--strict` makes warnings fail too. In GitHub Actions, the todography
action runs it:

```yaml
name: map

on:
  push:
    branches: [main]
  pull_request:

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: mkmik/todography@v0.12.0
        with:
          args: --strict
```

The action builds the todography of the version you pin, so the first run
takes a minute or two; later runs reuse it from the cache. Each error and
warning also shows up as a note on the file in the pull request.

The action takes three inputs, all optional:

- `command`: the todography command to run. The default is `lint`.
- `args`: what goes after the command, such as `--strict`.
- `map`: the map directory, or the directory above `docs/map`. Without
  it, todography looks for the map from the top of the repository.

Outside GitHub Actions, install todography and run it yourself:

```sh
cargo install todography --locked
todography lint --strict
```

Start without `--strict` if your map has warnings you aren't ready to fix,
and switch it on once it's clean.

## Publish the page

The page is one static file, so any static host will do. On GitHub Pages,
add a job that builds it and deploys it from `main`:

```yaml
  pages:
    needs: lint
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pages: write
      id-token: write
    environment:
      name: github-pages
      url: ${{ steps.deploy.outputs.page_url }}
    steps:
      - uses: actions/checkout@v4
      - uses: mkmik/todography@v0.12.0
        with:
          command: build
          args: --out site/index.html
      - uses: actions/upload-pages-artifact@v3
        with:
          path: site
      - id: deploy
        uses: actions/deploy-pages@v4
```

In the repository's settings, under Pages, set the source to GitHub
Actions.

The page records when it was built. To make builds reproducible, set
`SOURCE_DATE_EPOCH`, for example to the time of the last commit. After the
action has run once in a job, later steps can call `todography` directly:

```yaml
      - run: SOURCE_DATE_EPOCH=$(git log -1 --format=%ct) todography build --out site/index.html
```

## A pre-commit hook

To catch problems before they reach CI, run lint in a git hook. In
`.git/hooks/pre-commit`, made executable:

```sh
#!/bin/sh
exec todography lint
```

## In this repository

todography's own CI does both for the example map: `cargo test` checks that
it lints clean under `--strict`, and the docs workflow builds its page and
publishes it next to this book. See
[`.github/workflows/docs.yml`](https://github.com/mkmik/todography/blob/main/.github/workflows/docs.yml).
