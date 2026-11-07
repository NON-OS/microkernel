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

//! Every request that names a surface for the sender's window asks first
//! whether the sender answers for it (server/handlers/answers.rs), before
//! the scene changes: OP_SCENE_SUBMIT, OP_SCENE_SUBMIT_LAYER, OP_PRESENT,
//! OP_PRESENT_LAYER, OP_PRESENT_LAYER_PARTS, OP_LAYER_CONTENT and
//! OP_LAYER_CONTENT_PARTS. The handlers make kernel calls and
//! are not built here, so their source is read; the rule they ask is
//! answer_rule.rs answers, proved in answer_rule_tests over a kernel double.

const ANSWERS: &str = include_str!("../../../../compositor/src/server/handlers/answers.rs");
const SCREENSHOT: &str =
    include_str!("../../../../compositor/src/server/handlers/screenshot.rs");

/// Each handler, the call in it that changes the scene, and how it refuses.
const HANDLERS: [(&str, &str, &str); 4] = [
    (
        include_str!("../../../../compositor/src/server/handlers/scene_submit.rs"),
        "scene_submit::submit_layer(",
        "answers_for(ctx, sender_pid, layer.surface_handle)",
    ),
    (
        include_str!("../../../../compositor/src/server/handlers/scene_submit_layer.rs"),
        "scene_submit::submit_keyed_layer(",
        "answers_for(ctx, sender_pid, layer.surface_handle)",
    ),
    (
        include_str!("../../../../compositor/src/server/handlers/present.rs"),
        "rule(&mut ctx.scene, changed)",
        "answers_for(ctx, sender, shown.handle)",
    ),
    (
        include_str!("../../../../compositor/src/server/handlers/layer_content.rs"),
        "ctx.scene.set_content(",
        "answers_for(ctx, sender, set.handle)",
    ),
];

fn at(src: &str, what: &str) -> usize {
    src.find(what).unwrap_or_else(|| panic!("{what} is not in the handler"))
}

#[test]
fn each_handler_asks_whether_the_sender_answers_before_the_scene_changes() {
    for (src, change, ask) in HANDLERS {
        let asked = at(src, ask);
        assert!(asked < at(src, change), "{ask} after {change}");
        assert_eq!(src.matches(change).count(), 1, "{change} once");
        let refusal = &src[asked..];
        let refused = at(refusal, "E_PERM");
        assert!(refused < at(refusal, change), "{ask} refused E_PERM before {change}");
    }
}

#[test]
fn a_present_a_present_layer_and_one_with_parts_all_go_through_the_asking_step() {
    let (src, _, _) = HANDLERS[2];
    assert_eq!(src.matches("take(ctx, sender, req, tx, (shown,").count(), 3);
    assert_eq!(src.matches("fn take(").count(), 1);
}

#[test]
fn a_layer_content_and_one_with_parts_both_go_through_the_asking_step() {
    let (src, _, _) = HANDLERS[3];
    assert_eq!(src.matches("take(ctx, sender, req, tx, (set,").count(), 2);
    assert_eq!(src.matches("fn take(").count(), 1);
}

#[test]
fn only_a_layer_content_taken_off_skips_the_question() {
    let (src, _, _) = HANDLERS[3];
    assert!(src.contains("set.handle != 0 && !answers_for(ctx, sender, set.handle)"));
    for (src, _, ask) in &HANDLERS[..3] {
        assert!(src.contains(&format!("if !{ask}")), "{ask} asked unconditionally");
    }
}

#[test]
fn the_question_is_answer_rule_s_with_the_screen_s_views_and_the_cache_named() {
    let asked = "answers(&mut Kernel, screen, ctx.attach.holds(handle), sender, handle)";
    assert!(ANSWERS.contains(asked));
    assert!(ANSWERS.contains("handle == ctx.surface_handle"));
    assert!(ANSWERS.contains("ctx.heads.iter().any(|h| h.handle == handle)"));
    assert!(!ANSWERS.contains("lookup("), "the question takes no cache slot");
}

#[test]
fn a_screenshot_asks_capture_target_before_it_attaches_or_writes() {
    let asked = at(SCREENSHOT, "if !super::answers::capture_into(ctx, sender, handle)");
    assert!(at(SCREENSHOT, "may_capture(sender, mk_cap_check)") < asked);
    assert!(asked < at(SCREENSHOT, "attach_argb(handle)"));
    assert!(at(&SCREENSHOT[asked..], "E_PERM") < at(&SCREENSHOT[asked..], "attach_argb("));
    let into = "capture_target(&mut Kernel, screen, ctx.attach.holds(handle), sender, handle)";
    assert!(ANSWERS.contains(into));
    assert_eq!(ANSWERS.matches("let screen = screen_view(ctx, handle);").count(), 2);
}
