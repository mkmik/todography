# JSON

`todography json` prints the whole map and everything worked out from it.
The page that `todography build` writes carries exactly this JSON, so
anything the page shows, you can get from here for your own scripts.

Labels are absolute, `t8y:/area/component` or
`t8y:/area/component#feature`. The one exception is a label in a map file
that doesn't parse: it stays as written, and lint reports it. Optional
values are `null` when absent.

## Top level

| Field | Type | Meaning |
| --- | --- | --- |
| `title` | string | The map's title |
| `generated` | string | When it was made, as `YYYY-MM-DDTHH:MM:SSZ` in UTC. `SOURCE_DATE_EPOCH`, when set, is used instead of the current time |
| `bands` | array | `{id, title}`, top to bottom |
| `milestones` | array | `{id, title, summary, refs}`, in order |
| `areas` | array | See below, sorted by id |
| `components` | array | See below, sorted by label |
| `work` | array | Every work item, wanted or not |
| `lint` | object | `{errors, warnings}`, each an array of `{file, msg}` |

## Areas

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | string | The directory's path under the map, such as `net/ip` |
| `title` | string | |
| `band` | string | Its band's id |
| `summary` | string | |
| `body_html` | string | The body of `AREA.md`, rendered from Markdown |

## Components

One component of the example map, as `todography json` prints it:

```json
{{#include ../transcripts/json-component.json}}
```

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | string | |
| `area` | string | Its area's id |
| `slug` | string | The file name without `.md` |
| `title`, `summary` | string | |
| `aka`, `refs` | array of strings | |
| `kind` | string | |
| `intent` | string | `goal`, `aspiration` or `induced` |
| `research` | string | `done`, `outline` or `deferred` |
| `milestone` | string or null | |
| `status` | string | The [worked-out status](computed.md#status) |
| `status_explicit` | bool | True when the file sets `status` |
| `body_html` | string | The body, rendered from Markdown |
| `features` | array | See below |
| `needs` | array | Its `[[needs]]`, then each feature's needs |
| `needed_by` | array | See below |
| `wanted` | bool | |
| `why_paths` | array of arrays of labels | Up to three chains, goal first |
| `debt` | bool | Research debt |
| `progress` | object | `{done, total}`: see [Progress](computed.md#progress) |

Markdown is rendered with tables, strikethrough and task lists.

### Features

| Field | Type | Meaning |
| --- | --- | --- |
| `id` | string | |
| `label` | string | The component's label with `#id` |
| `title` | string | |
| `status` | string | `done`, `wip`, `todo` or `dropped` |
| `milestone`, `note` | string or null | As written; the component's milestone isn't filled in |
| `refs` | array of strings | |
| `needs` | array of labels | Absolute where they parse |

### Needs

| Field | Type | Meaning |
| --- | --- | --- |
| `on` | string or null | The label, made absolute; as written if it doesn't parse |
| `fuzzy` | string or null | |
| `why` | string or null | |
| `soft` | bool | |
| `from_feature` | string or null | The feature it comes from, for a feature's need |
| `target` | label or null | The component it resolves to |
| `target_feature` | string or null | The feature it points at |
| `met` | bool | |

### Needed by

| Field | Type | Meaning |
| --- | --- | --- |
| `from` | label | The component that needs this one |
| `from_feature` | string or null | Its feature the need comes from |
| `on_feature` | string or null | This component's feature it points at |
| `why` | string or null | |
| `soft` | bool | |

## Work

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | string | The feature's label, or the component's for a component without features |
| `component` | label | |
| `feature` | string or null | The feature id |
| `title` | string | |
| `status` | string | |
| `milestone` | string or null | The feature's milestone, or else its component's |
| `wanted` | bool | |
| `ready` | bool | |
| `blocked_by` | array of labels | Its unmet hard needs and its component's |

## Scripting against it

The page reads exactly these fields, so they change together with the
page. If you write scripts against the JSON, pin the todography version
you use and check the release notes when you upgrade.
