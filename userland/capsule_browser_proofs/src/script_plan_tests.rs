// NONOS Operating System (AGPL-3.0-or-later)
//! A page's scripts run in document order, inline and external alike, and
//! deferred ones after.

use crate::browser::dom::parse;
use crate::browser::js::script_plan::{next_ready, plan, script_kind, PageScript, ScriptKind};
use crate::browser::js::script_plan::{MAX_EXTERNAL, MAX_STEPS};

fn ext(s: &str) -> PageScript {
    PageScript::External(alloc::string::String::from(s))
}

fn inline(s: &str) -> PageScript {
    PageScript::Inline(alloc::string::String::from(s))
}

#[test]
fn inline_and_external_scripts_keep_document_order() {
    let dom = parse(
        b"<html><head><script src=jquery.js></script><script>$(go)</script></head>\
          <body><script>a()</script><script src=late.js></script><script>b()</script></body></html>",
    );
    assert_eq!(
        plan(&dom),
        [ext("jquery.js"), inline("$(go)"), inline("a()"), ext("late.js"), inline("b()")],
        "the inline call to the library comes after the library"
    );
}

#[test]
fn deferred_and_module_scripts_run_after_the_rest_in_their_own_order() {
    let dom = parse(
        b"<script src=d1.js defer></script><script type=module>m()</script>\
          <script src=now.js></script><script src=a.js async></script>\
          <script type=module src=m.js></script><script defer>inline_defer_is_ignored()</script>",
    );
    assert_eq!(
        plan(&dom),
        [
            ext("now.js"),
            ext("a.js"),
            inline("inline_defer_is_ignored()"),
            ext("d1.js"),
            inline("m()"),
            ext("m.js"),
        ]
    );
}

#[test]
fn data_kept_in_script_elements_is_not_run() {
    let dom = parse(
        b"<script type=application/json>{\"a\":1}</script>\
          <script type=application/ld+json>{}</script>\
          <script type=text/template><p></p></script>\
          <script type=importmap>{}</script>\
          <script type=\"text/javascript; charset=utf-8\">ok1()</script>\
          <script language=javascript>ok2()</script>\
          <script type=\"\">ok3()</script>\
          <script src=\"\">never()</script>",
    );
    assert_eq!(plan(&dom), [inline("ok1()"), inline("ok2()"), inline("ok3()")]);
}

#[test]
fn the_script_types_follow_the_standard() {
    assert_eq!(script_kind(None, None), Some(ScriptKind::Classic));
    assert_eq!(script_kind(Some(""), None), Some(ScriptKind::Classic));
    assert_eq!(script_kind(Some(" Module "), None), Some(ScriptKind::Module));
    assert_eq!(script_kind(Some("TEXT/JAVASCRIPT"), None), Some(ScriptKind::Classic));
    assert_eq!(script_kind(Some("application/x-javascript"), None), Some(ScriptKind::Classic));
    assert_eq!(script_kind(None, Some("JavaScript1.2")), Some(ScriptKind::Classic));
    assert_eq!(script_kind(None, Some("vbscript")), None);
    assert_eq!(script_kind(Some("text/babel"), None), None);
    assert_eq!(script_kind(Some("speculationrules"), None), None);
}

#[test]
fn the_plan_is_bounded() {
    let mut html = alloc::string::String::new();
    for i in 0..MAX_EXTERNAL + 5 {
        html.push_str(&alloc::format!("<script src=s{i}.js></script>"));
    }
    let externals = plan(&parse(html.as_bytes())).len();
    assert_eq!(externals, MAX_EXTERNAL);
    let many = "<script>x()</script>".repeat(MAX_STEPS + 20);
    assert_eq!(plan(&parse(many.as_bytes())).len(), MAX_STEPS);
}

#[test]
fn a_script_runs_only_when_its_turn_has_come() {
    let held = alloc::vec![(2u32, b"c".to_vec()), (0, b"a".to_vec())];
    assert_eq!(next_ready(&held, 0), Some(1));
    assert_eq!(next_ready(&held, 1), None, "the one before has not landed");
    assert_eq!(next_ready(&held, 2), Some(0));
}
