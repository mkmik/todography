<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo/todography-logo-dark.svg">
    <img alt="todography" src="assets/logo/todography-logo.svg" width="390">
  </picture>
</p>

# todography

Issue trackers are good at handing out tasks and bad at telling you where a
project stands. Which parts are solid, which are thin, and what do you have
to build before the thing you actually want? I wanted a map for that, kept
in the repository and edited in the same commits as the code, so it stays
true.

So todography is that map. Each component of your project is a small
Markdown file under `docs/map/` that says what it does, how far along it
is, and what it needs. The tool checks those files, works out what's
wanted, what's ready and what's blocked, and draws it all as a page you can
click through.

```sh
cargo install todography

todography lint        # is the map consistent?
todography next --all  # what's ready, and what's blocked by what
todography build       # write the interactive page
```

The name is *todo* plus *topography*.

## Documentation

Everything is in the book: **https://mkmik.github.io/todography/**

It starts with a small example, then walks through todography's own map,
which lives in [`examples/todography`](examples/todography/docs/map). You
can [see that map's page](https://mkmik.github.io/todography/example/)
without installing anything.

## Status

It's young. I wrote it for vaxpunk, an OpenVMS remake, and it still
changes as I learn what a map needs.

## License

MIT
