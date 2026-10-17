//! Checks the example map in examples/todography: todography's map of itself.
//!
//! The documentation walks through this map, so these tests do three jobs.
//! They keep the map lint-clean under --strict. They pin down every fact the
//! walkthrough states in prose. And they keep the command output the book
//! shows, in docs/src/transcripts/, identical to what the tool prints.
//!
//! When a change to the map or the tool changes that output on purpose, run
//!
//!     TODOGRAPHY_BLESS=1 cargo test --test example
//!
//! to rewrite the transcripts, then read the walkthrough pages that include
//! them (docs/src/walkthrough/) for any sentence that now says the old thing.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use todography::Map;
use todography::map::{Comp, Intent, Status};
use todography::{json, query};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/todography")
}

fn dir() -> PathBuf {
    root().join("docs/map")
}

fn load() -> Map {
    Map::load(&dir())
}

fn comp<'a>(m: &'a Map, path: &str) -> &'a Comp {
    let i = *m
        .by_path
        .get(path)
        .unwrap_or_else(|| panic!("the example has no component {path}"));
    &m.comps[i]
}

fn paths(m: &Map, pick: impl Fn(&Comp) -> bool) -> Vec<String> {
    let mut v: Vec<String> = m
        .comps
        .iter()
        .filter(|c| pick(c))
        .map(|c| c.path.clone())
        .collect();
    v.sort();
    v
}

/// Runs the binary on the example from the repository root.
fn run(map: &Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_todography"))
        .args(["--map", map.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8(out.stdout).unwrap(),
    )
}

/// A scratch copy of the example, for the walkthrough's "what if" steps.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let dir =
            std::env::temp_dir().join(format!("todography-example-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        copy_dir(&root().join("docs"), &dir.join("docs"));
        Scratch(dir)
    }
    fn edit(&self, file: &str, from: &str, to: &str) {
        let path = self.0.join("docs/map").join(file);
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains(from), "{file} has no {from:?}");
        fs::write(&path, text.replacen(from, to, 1)).unwrap();
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        let dest = to.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &dest);
        } else {
            fs::copy(e.path(), dest).unwrap();
        }
    }
}

#[test]
fn lints_clean() {
    let m = load();
    assert_eq!(query::lint(&m).lines().count(), 1, "{}", query::lint(&m));
    assert!(m.lint.errors.is_empty());
    assert!(m.lint.warnings.is_empty());
    let (code, out) = run(&root(), &["lint", "--strict"]);
    assert_eq!(code, 0, "{out}");
}

#[test]
fn shape() {
    let m = load();
    assert_eq!(m.title, "todography");
    let bands: Vec<_> = m.bands.iter().map(|b| b.id.as_str()).collect();
    assert_eq!(bands, ["surfaces", "engine", "format", "ship", "deps"]);
    let areas: Vec<_> = m.areas.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(
        areas,
        [
            "bridges", "cli", "crates", "format", "model", "page", "ship", "tui"
        ]
    );
    assert!(m.milestones.is_empty(), "the walkthrough says it has none");
}

#[test]
fn goals_pull_in_the_rest() {
    let m = load();
    // Every component is wanted except the aspirations.
    assert_eq!(
        paths(&m, |c| c.intent == Intent::Aspiration),
        [
            "bridges/jira",
            "format/tags",
            "model/buckets",
            "page/canvas-layout"
        ]
    );
    assert_eq!(
        paths(&m, |c| !c.wanted),
        paths(&m, |c| c.intent == Intent::Aspiration)
    );
    // The walkthrough lists the goals by name.
    assert_eq!(
        paths(&m, |c| c.intent == Intent::Goal),
        [
            "cli/build",
            "cli/queries",
            "format/lint",
            "page/agent-prompt",
            "page/choose",
            "page/focus",
            "page/map-view",
            "page/roadmap",
            "ship/ci",
            "ship/docs",
            "ship/release",
            "ship/renovate",
            "tui/focus",
            "tui/map",
            "tui/roadmap"
        ]
    );
    // The model area is all induced: nobody wants it for itself.
    for c in m.comps.iter().filter(|c| c.area == "model" && c.wanted) {
        assert_eq!(c.intent, Intent::Induced, "{}", c.path);
    }
    // base64 is on the map because the terminal copies over OSC 52.
    let b = comp(&m, "crates/base64");
    assert_eq!(b.why_paths.len(), 3);
    let terminal = m.by_path["tui/terminal"];
    for p in &b.why_paths {
        assert_eq!(
            p[p.len() - 2],
            terminal,
            "every chain ends terminal -> base64"
        );
        assert_eq!(m.comps[p[0]].intent, Intent::Goal);
    }
    let (_, out) = run(&root(), &["why", "crates/base64"]);
    assert!(
        out.contains("t8y:/tui/terminal -> t8y:/crates/base64"),
        "{out}"
    );
}

#[test]
fn status_and_met() {
    let m = load();
    // Three features done and one todo: partial.
    assert_eq!(comp(&m, "page/agent-prompt").status, Status::Partial);
    // JSON has a todo feature, so it's partial too, but a need on the whole
    // component is met once it's partial.
    let json = comp(&m, "cli/json");
    assert_eq!(json.status, Status::Partial);
    let shell = comp(&m, "page/shell");
    let on_json = shell
        .needs
        .iter()
        .find(|n| n.on.as_deref() == Some("t8y:/cli/json"))
        .unwrap();
    assert!(on_json.met);
    // A component without features takes its status field.
    assert_eq!(comp(&m, "model/wanted").status, Status::Done);
    assert!(comp(&m, "model/wanted").features.is_empty());
    // The walkthrough says the example has no soft needs.
    assert!(m.comps.iter().all(|c| c.needs.iter().all(|n| !n.soft)));
}

#[test]
fn next_work() {
    let m = load();
    let wanted: Vec<_> = m.work.iter().filter(|w| w.wanted).collect();
    assert_eq!(wanted.len(), 2);
    let ready: Vec<_> = wanted
        .iter()
        .filter(|w| w.ready)
        .map(|w| w.label.as_str())
        .collect();
    assert_eq!(ready, ["t8y:/cli/json#prompts"]);
    let blocked = wanted.iter().find(|w| !w.ready).unwrap();
    assert_eq!(blocked.label, "t8y:/page/agent-prompt#one-source");
    assert_eq!(blocked.blocked_by, ["t8y:/cli/json#prompts"]);

    let (_, out) = run(&root(), &["next"]);
    assert!(out.ends_with("1 ready\n"), "{out}");
    let (_, out) = run(&root(), &["next", "--all"]);
    assert!(out.ends_with("1 ready, 1 blocked\n"), "{out}");
    assert!(out.contains("blocked by t8y:/cli/json#prompts"), "{out}");
}

#[test]
fn aspirations_carry_their_debt() {
    let m = load();
    assert!(m.comps.iter().all(|c| !c.debt), "nothing wanted has debt");
    for c in m.comps.iter().filter(|c| c.intent == Intent::Aspiration) {
        assert_eq!(c.research.as_str(), "deferred", "{}", c.path);
        assert!(c.needs.iter().any(|n| n.fuzzy.is_some()), "{}", c.path);
    }
    let (_, out) = run(&root(), &["debt"]);
    assert_eq!(out, "research debt: 0\n");
}

#[test]
fn choosing_the_jira_bridge() {
    // The walkthrough's "what if": make the Jira bridge a goal.
    let s = Scratch::new("jira");
    s.edit(
        "bridges/jira.md",
        "intent = \"aspiration\"",
        "intent = \"goal\"",
    );
    let m = Map::load(&s.0.join("docs/map"));
    assert!(m.lint.errors.is_empty() && m.lint.warnings.is_empty());
    let jira = comp(&m, "bridges/jira");
    assert!(jira.wanted && jira.debt);
    assert_eq!(paths(&m, |c| c.debt), ["bridges/jira"]);
    let (_, out) = run(&s.0, &["debt"]);
    assert!(out.contains("fuzzy: Jira's REST API"), "{out}");
    assert!(out.ends_with("research debt: 1\n"), "{out}");
    // Fuzzy needs never block: it's ready at once.
    let (_, out) = run(&s.0, &["next"]);
    assert!(out.contains("t8y:/bridges/jira"), "{out}");
    assert!(out.ends_with("2 ready\n"), "{out}");
}

#[test]
fn finishing_the_blocker() {
    // The walkthrough's other "what if": land the JSON prompts.
    let s = Scratch::new("prompts");
    s.edit(
        "cli/json.md",
        "id = \"prompts\"\ntitle = \"Each work item carries its agent prompt, written once in Rust\"\nstatus = \"todo\"",
        "id = \"prompts\"\ntitle = \"Each work item carries its agent prompt, written once in Rust\"\nstatus = \"done\"",
    );
    let m = Map::load(&s.0.join("docs/map"));
    assert_eq!(comp(&m, "cli/json").status, Status::Done);
    let ready: Vec<_> = m
        .work
        .iter()
        .filter(|w| w.ready)
        .map(|w| w.label.as_str())
        .collect();
    assert_eq!(ready, ["t8y:/page/agent-prompt#one-source"]);
}

#[test]
fn builds_json_page_and_tui() {
    let m = load();
    let doc: serde_json::Value = serde_json::from_str(&json::to_json(&m)).unwrap();
    assert_eq!(doc["title"], "todography");
    assert_eq!(doc["components"].as_array().unwrap().len(), m.comps.len());
    let page = json::page(&m);
    // The example's own Build component mentions the placeholder, so look
    // for the unfilled line rather than the word.
    assert!(!page.contains("const MAP = __MAP_DATA__;"));
    assert!(page.contains("t8y:/page/agent-prompt#one-source"));

    use ratatui::{Terminal, backend::TestBackend};
    use todography::tui::App;
    let mut app = App::new(&dir());
    let mut term = Terminal::new(TestBackend::new(120, 40)).unwrap();
    term.draw(|f| app.draw(f)).unwrap();
}

/// A transcript the book includes: a command, run on the example or on a
/// scratch copy with one edit, and what it printed.
struct Transcript {
    name: &'static str,
    /// (file, from, to): an edit to make in a scratch copy first.
    edit: Option<(&'static str, &'static str, &'static str)>,
    args: &'static [&'static str],
}

const CHOOSE_JIRA: (&str, &str, &str) = (
    "bridges/jira.md",
    "intent = \"aspiration\"",
    "intent = \"goal\"",
);
const LAND_PROMPTS: (&str, &str, &str) = (
    "cli/json.md",
    "written once in Rust\"\nstatus = \"todo\"",
    "written once in Rust\"\nstatus = \"done\"",
);
const TYPO: (&str, &str, &str) = (
    "tui/roadmap.md",
    "on = \"/model/work\"",
    "on = \"/model/wrok\"",
);

const TRANSCRIPTS: &[Transcript] = &[
    Transcript {
        name: "help",
        edit: None,
        args: &["help"],
    },
    Transcript {
        name: "lint",
        edit: None,
        args: &["lint", "--strict"],
    },
    Transcript {
        name: "why-base64",
        edit: None,
        args: &["why", "crates/base64"],
    },
    Transcript {
        name: "show-agent-prompt",
        edit: None,
        args: &["show", "page/agent-prompt"],
    },
    Transcript {
        name: "show-one-source",
        edit: None,
        args: &["show", "page/agent-prompt#one-source"],
    },
    Transcript {
        name: "show-json",
        edit: None,
        args: &["show", "cli/json"],
    },
    Transcript {
        name: "show-jira",
        edit: None,
        args: &["show", "bridges/jira"],
    },
    Transcript {
        name: "rdeps-prompts",
        edit: None,
        args: &["rdeps", "cli/json#prompts"],
    },
    Transcript {
        name: "rdeps-all-graph",
        edit: None,
        args: &["rdeps", "--all", "model/graph"],
    },
    Transcript {
        name: "next",
        edit: None,
        args: &["next"],
    },
    Transcript {
        name: "next-all",
        edit: None,
        args: &["next", "--all"],
    },
    Transcript {
        name: "debt",
        edit: None,
        args: &["debt"],
    },
    Transcript {
        name: "jira-debt",
        edit: Some(CHOOSE_JIRA),
        args: &["debt"],
    },
    Transcript {
        name: "jira-next",
        edit: Some(CHOOSE_JIRA),
        args: &["next"],
    },
    Transcript {
        name: "prompts-done-next",
        edit: Some(LAND_PROMPTS),
        args: &["next", "--all"],
    },
    Transcript {
        name: "typo-lint",
        edit: Some(TYPO),
        args: &["lint"],
    },
];

/// The getting-started page's map, in examples/quickstart: (name, args).
const QUICKSTART: &[(&str, &[&str])] = &[
    ("quickstart-lint", &["lint"]),
    ("quickstart-next-all", &["next", "--all"]),
    ("quickstart-show-sync", &["show", "core/sync"]),
    ("quickstart-why-watch", &["why", "core/watch"]),
];

fn quickstart() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/quickstart")
}

#[test]
fn quickstart_lints_clean() {
    let m = Map::load(&quickstart().join("docs/map"));
    assert!(m.lint.errors.is_empty(), "{}", query::lint(&m));
    assert!(m.lint.warnings.is_empty(), "{}", query::lint(&m));
    assert_eq!(m.comps.len(), 2);
}

fn transcripts_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/src/transcripts")
}

/// Compares `text` with the transcript file `name`, or writes it when
/// TODOGRAPHY_BLESS is set. Returns a complaint on a mismatch.
fn check_transcript(name: &str, text: &str) -> Option<String> {
    let path = transcripts_dir().join(name);
    if std::env::var_os("TODOGRAPHY_BLESS").is_some() {
        fs::create_dir_all(transcripts_dir()).unwrap();
        fs::write(&path, text).unwrap();
        return None;
    }
    match fs::read_to_string(&path) {
        Ok(old) if old == text => None,
        Ok(old) => Some(format!("{name} changed:\n--- book\n{old}\n--- now\n{text}")),
        Err(_) => Some(format!("{name} is missing")),
    }
}

#[test]
fn transcripts_match_the_book() {
    let mut problems = vec![];
    for t in TRANSCRIPTS {
        let scratch;
        let map = match t.edit {
            None => root(),
            Some((file, from, to)) => {
                scratch = Scratch::new(t.name);
                scratch.edit(file, from, to);
                scratch.0.clone()
            }
        };
        let (_, out) = run(&map, t.args);
        let text = format!("$ todography {}\n{out}", t.args.join(" "));
        problems.extend(check_transcript(&format!("{}.txt", t.name), &text));
    }

    for (name, args) in QUICKSTART {
        let (_, out) = run(&quickstart(), args);
        let text = format!("$ todography {}\n{out}", args.join(" "));
        problems.extend(check_transcript(&format!("{name}.txt"), &text));
    }

    // What the copy button puts on the clipboard for the blocked work.
    let app = todography::tui::App::new(&dir());
    let k = app
        .map
        .work
        .iter()
        .position(|w| w.label == "t8y:/page/agent-prompt#one-source")
        .unwrap();
    problems.extend(check_transcript(
        "prompt-one-source.txt",
        &(app.prompt(k) + "\n"),
    ));

    // One component as `todography json` writes it, cut out of the real
    // output so the fields keep their order.
    let out = json::to_json(&load());
    let lines: Vec<&str> = out.lines().collect();
    let at = lines
        .iter()
        .position(|l| *l == "      \"label\": \"t8y:/ship/tests\",")
        .unwrap();
    assert_eq!(lines[at - 1], "    {");
    let end = at
        + lines[at..]
            .iter()
            .position(|l| l.starts_with("    }"))
            .unwrap();
    let text: String = lines[at - 1..=end]
        .iter()
        .map(|l| format!("{}\n", l.strip_prefix("    ").unwrap_or(l)))
        .collect();
    // The last line closes the component; drop the comma that follows it
    // in the full output.
    let text = format!("{}\n", text.trim_end().trim_end_matches(','));
    serde_json::from_str::<serde_json::Value>(&text).expect("the excerpt is JSON");
    problems.extend(check_transcript("json-component.json", &text));

    assert!(
        problems.is_empty(),
        "{}\n\nIf the change is intended: TODOGRAPHY_BLESS=1 cargo test --test example",
        problems.join("\n\n")
    );
}
