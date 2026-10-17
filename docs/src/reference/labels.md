# Labels

A label names a component, or one feature of a component. Labels are URIs
with the scheme `t8y`:

| Label | Names |
| --- | --- |
| `t8y:/exec/mailboxes` | The component in `docs/map/exec/mailboxes.md` |
| `t8y:/exec/mailboxes#attention-asts` | Its feature with id `attention-asts` |
| `t8y:/net/ip/tcp` | The component in `docs/map/net/ip/tcp.md` |

The path is the file's path under the map directory, without `.md`. Each
part of the path, and the feature id, is lowercase letters, digits and
`-`, starting with a letter or digit.

## In map files

Inside a map file, short forms are relative references in the sense of
RFC 3986, resolved against the file's own label. Any URI library reads them
the same way.

| Written in `docs/map/exec/mailboxes.md` | Means |
| --- | --- |
| `#attention-asts` | `t8y:/exec/mailboxes#attention-asts` |
| `/net/ssl` | `t8y:/net/ssl` |
| `/net/ssl#sockets` | `t8y:/net/ssl#sockets` |
| `t8y:/net/ssl` | `t8y:/net/ssl` |

These are refused:

| Written | Why |
| --- | --- |
| `qio`, `../net/ssl` | Relative paths change meaning when a file moves |
| `//exec/qio` | An old form; write `t8y:/exec/qio` or `/exec/qio` |
| `t8y:#basic` | A bare fragment can't have a scheme |
| `/exec/Mailboxes` | Upper case |
| `/exec//mailboxes`, `/` | Empty parts |
| `/exec/mailboxes#` | An empty feature id |

A label must name something that exists. One that doesn't is an error,
and lint suggests the closest real label when there is one close enough to
be a typo. Labels into a component whose own file failed to parse are
checked only as far as the component; its features can't be checked until
the file parses.

## On the command line

Commands that take a label accept the absolute forms and a bare path. A
bare `#feature` has nothing to be relative to there, so it isn't accepted.
Quoting a label that has a `#` is a good habit, since some shells treat
`#` specially:

```console
todography show exec/mailboxes
todography show /exec/mailboxes
todography show t8y:/exec/mailboxes
todography show 'exec/mailboxes#attention-asts'
```

## Everywhere else

In prose, code comments, commit messages and design documents, write the
full form, `t8y:/exec/mailboxes`. Then `grep -r t8y:` finds every
reference to the map in the repository, and a reader knows it's a map
label and not a file path.
