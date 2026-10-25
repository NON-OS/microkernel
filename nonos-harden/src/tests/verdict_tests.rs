// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The GO rule and the verdict's two forms: GO needs every check to have run
//! and passed and the kind of image known; the text form cannot be steered by
//! what it quotes; the JSON form names its format.

use crate::check::Check;
use crate::ctx::{Ctx, Kind};
use crate::verdict::{build, decide, json, text, Overall, DEV_GO, FORMAT};

fn ok(id: &'static str) -> Check {
    Check::pass(id, "fine")
}

#[test]
fn go_needs_every_check_run_and_passed_and_the_kind_known() {
    assert_eq!(decide(Kind::Release, &[ok("a"), ok("b")]), (Overall::Go, "GO".into()));
    assert_eq!(decide(Kind::Development, &[ok("a")]), (Overall::Go, DEV_GO.into()));
    let failed = decide(Kind::Release, &[ok("a"), Check::fail("b", "x"), Check::fail("c", "y")]);
    assert_eq!(failed, (Overall::NoGo, "NO-GO: 2 of 3 checks did not pass; first b".into()));
    let unrun = decide(Kind::Development, &[ok("a"), Check::not_run("b", "no tree")]);
    assert_eq!(unrun.0, Overall::NoGo, "a check that did not run is no pass");
    assert_eq!(decide(Kind::Release, &[]).0, Overall::NoGo);
    assert_eq!(decide(Kind::Unknown, &[ok("a")]).0, Overall::NoGo);
}

#[test]
fn the_text_verdict_cannot_be_steered_by_what_it_quotes() {
    let mut ctx = Ctx::new("rel\u{1b}[2J".into(), None);
    ctx.kind = Kind::Development;
    let quoted = "\u{1b}]0;GO\u{7}x\u{202e}gnp.exe\r\nGO";
    let c = Check::fail("release.sealed", quoted).with_limits(vec!["\u{2066}hidden".into()]);
    let out = text::render(&build(&ctx, vec![c], None));
    for bad in ['\u{1b}', '\u{7}', '\r', '\u{202e}', '\u{2066}'] {
        assert!(!out.contains(bad), "{bad:?} reached the terminal");
    }
    let tail =
        "\nNO-GO: 1 of 1 checks did not pass; first release.sealed\n  limit: release.sealed: ";
    assert!(out.contains(tail) && out.ends_with("hidden\n"), "{out}");
}

#[test]
fn the_json_verdict_names_its_format_kind_and_every_check() {
    let mut ctx = Ctx::new("r".into(), None);
    ctx.kind = Kind::Development;
    let v = build(
        &ctx,
        vec![ok("a"), Check::not_run("b", "no tree").with_limits(vec!["l".into()])],
        None,
    );
    let j: serde_json::Value = serde_json::from_str(&json::render(&v).unwrap()).unwrap();
    assert_eq!(j["format"], FORMAT);
    assert_eq!((j["overall"].as_str(), j["kind"].as_str()), (Some("no-go"), Some("development")));
    assert_eq!(j["checks"][1]["status"], "not-run");
    assert_eq!(j["checks"][1]["limits"][0], "l");
    assert!(j["checks"][0].get("limits").is_none(), "an empty list is left out");
}
