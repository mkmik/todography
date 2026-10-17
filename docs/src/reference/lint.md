# Lint

`todography lint` reads the whole map and reports every problem it finds.
Loading never stops at the first problem: each file is read as far as it
can be, and everything wrong goes into the report.

## Output

```text
docs/map/exec/qio.md: error: unresolved label /exec/timer: no component t8y:/exec/timer; did you mean t8y:/exec/timers?
docs/map/net/ssl.md: warning: research is missing; taken as deferred
1 errors, 1 warnings, 42 components, research debt: 3
```

Each line is `file: error: message` or `file: warning: message`. File
names are relative to the workspace root: the directory above `docs/` when
the map is at `docs/map`, so they start with `docs/map/`. A map anywhere
else is named by its own directory, as in `map/exec/qio.md`.
Errors come first, then warnings, each sorted by file and in the order they
were found within a file. The last line counts errors, warnings,
components, and wanted components with research debt.

## Exit status

| Status | When |
| --- | --- |
| 0 | No errors (and, with `--strict`, no warnings) |
| 1 | Errors (or, with `--strict`, warnings) |
| 2 | No map was found, or the command line was wrong |

Without `--map`, "no map found" means no `todography.toml` here or above.
With `--map DIR`, todography doesn't check that `DIR` is a map: a directory
that is missing or empty shows up as lint errors, and exit status 1.

## Errors

### todography.toml

| Message | Cause |
| --- | --- |
| `can't read it: ...` | The file is missing or unreadable |
| `line N: ...` | The TOML doesn't parse, has an unknown field, or misses a required one. The message is the TOML parser's |
| `missing title` | `title` is missing or blank |
| `bad id "x": use lowercase letters, digits and -` | A band, milestone or kind id isn't a valid id |
| `duplicate id "x"` | Two bands or milestones share an id, or two kinds do |

### Directories

| Message | Cause |
| --- | --- |
| `directory without AREA.md` | Every directory under the map is an area and needs one |
| `bad directory name: use lowercase letters, digits and -` | The directory's name can't be part of a label |
| `can't read the directory: ...` | A filesystem error |
| `a component must sit in an area's directory` | A `.md` file directly in the map directory. `README.md` and `AREA.md` there are skipped |

### AREA.md and component files

| Message | Cause |
| --- | --- |
| `no front matter: the first line must be +++` | The file doesn't start with `+++` |
| `the front matter has no closing +++ line` | No second `+++` line |
| `line N: ...` | The front matter doesn't parse, has an unknown field, a value that isn't allowed (such as `intent = "wanted"`), or a feature without an `id` |
| `missing title` | `title` is missing or blank |
| `missing summary` | `summary` is missing or blank |

### AREA.md

| Message | Cause |
| --- | --- |
| `missing band` | No `band` field |
| `unknown band "x"` | The band isn't in `todography.toml` |

### Components

| Message | Cause |
| --- | --- |
| `bad file name: use lowercase letters, digits and -, then .md` | The file's name can't be part of a label |
| `can't read it: ...` | A filesystem error. (An `AREA.md` that can't be read is reported as `directory without AREA.md`.) |
| `missing kind` | No `kind` field |
| `unknown kind "x"` | The kind isn't in `todography.toml` |
| `missing intent` | No `intent` field |
| `unknown milestone "x"` | The component's milestone isn't in `todography.toml` |
| `status is set, but it has features: its status comes from them` | Remove `status`, or the features |
| `a need has both on and fuzzy: pick one` | |
| `a need has neither on nor fuzzy` | |

### Features

| Message | Cause |
| --- | --- |
| `bad feature id "x": use lowercase letters, digits and -` | |
| `duplicate feature id "x"` | Two features of one component share an id |
| `feature x: missing title` | |
| `feature x: missing status` | |
| `feature x: unknown milestone "m"` | |

### Labels

| Message | Cause |
| --- | --- |
| `bad label "x": ...` | The label isn't one of the [accepted forms](labels.md) |
| `unresolved label x: no component t8y:/a/b` | No such file. Followed by `; did you mean t8y:/a/c?` when a label is close |
| `unresolved label x: t8y:/a/b has no feature #f` | No such feature. Followed by `; did you mean #g?` when an id is close |
| `x: a component can't need itself` | A component's `[[needs]]` on its own label or one of its own features, a feature needing its own whole component, or a feature needing itself. A feature may need a different feature of its own component |

"Close" means a small edit distance: one character for short names, up to
three for long ones.

## Warnings

| Message | Cause |
| --- | --- |
| `summary is N characters; keep it to 100` | On an area or a component |
| `research is missing; taken as deferred` | No `research` field |
| `induced, but nothing needs it` | An induced component that isn't dropped, and that no other component needs |
| `an aspiration, but wanted: t8y:/a/b needs it; make it induced or a goal` | Something wanted hard-needs it, so it isn't an unchosen idea any more |
| `dependency cycle: t8y:/a -> t8y:/b -> t8y:/a` | Components that hard-need each other in a loop |

For cycles, todography finds each group of components that hard-need each
other in a loop and reports one shortest cycle through it, starting at the
first component by label, in that component's file. A feature that needs
another feature of its own component isn't a cycle.

Components whose file failed to parse are left out of these checks.
