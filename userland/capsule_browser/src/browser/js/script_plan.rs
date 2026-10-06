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

//! The order a page's scripts run in, decided once from its markup.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

/// External scripts one page may fetch.
pub const MAX_EXTERNAL: usize = 24;
/// Scripts of any kind one page runs.
pub const MAX_STEPS: usize = 1000;

/// One script to run, in its turn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageScript {
    /// The text of an inline `<script>`.
    Inline(String),
    /// The `src` of an external one, as markup spelled it.
    External(String),
}

/// How a `<script>` element's type says it runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptKind {
    Classic,
    Module,
}

/*
 * All inline scripts used to run the moment the page committed, and every
 * external one later as it arrived, so a page's inline code ran before the
 * library it calls had loaded, and threw. The HTML standard runs them in
 * two passes over the document: the parser-inserted scripts in document
 * order (async ones may run whenever they arrive, and document order is one
 * such time), then the deferred ones, `defer` with a `src` and every module,
 * in document order. The whole tree is already parsed here, so a script in
 * the first pass sees all of it rather than the part above it; that is the
 * one difference left, and pages that wait for DOMContentLoaded do not see
 * it at all.
 */
/// The page's scripts in the order they run.
pub fn plan(dom: &Dom) -> Vec<PageScript> {
    let mut now = Vec::new();
    let mut deferred = Vec::new();
    let mut external = 0usize;
    for node in &dom.nodes {
        if node.kind != NodeKind::Element || node.tag != "script" {
            continue;
        }
        let Some(kind) = script_kind(node.attr("type"), node.attr("language")) else {
            continue;
        };
        let step = match node.attr("src") {
            /* A src that is present but empty fires error and runs nothing. */
            Some("") => continue,
            Some(src) if external < MAX_EXTERNAL => {
                external += 1;
                PageScript::External(String::from(src))
            }
            Some(_) => continue,
            None => PageScript::Inline(inline_text(dom, &node.children)),
        };
        let defer = node.attr("defer").is_some() && matches!(step, PageScript::External(_));
        if kind == ScriptKind::Module || defer {
            deferred.push(step);
        } else {
            now.push(step);
        }
        if now.len() + deferred.len() >= MAX_STEPS {
            break;
        }
    }
    now.extend(deferred);
    now
}

/// Where in `held` the script whose turn is `run_order` sits, if it has
/// arrived. Scripts land in any order; they run only in this one.
pub fn next_ready(held: &[(u32, Vec<u8>)], run_order: u32) -> Option<usize> {
    held.iter().position(|(order, _)| *order == run_order)
}

fn inline_text(dom: &Dom, children: &[usize]) -> String {
    let mut out = String::new();
    for &c in children {
        if let Some(n) = dom.nodes.get(c).filter(|n| n.kind == NodeKind::Text) {
            out.push_str(&n.text);
        }
    }
    out
}

/*
 * HTML "prepare the script element" steps 8 to 10: no type and no language
 * is JavaScript, an empty type is JavaScript, "module" is a module, and
 * otherwise the type has to be one of the JavaScript MIME types. Anything
 * else is data a page keeps in a script element (JSON, JSON-LD, templates,
 * import maps) and running it would only throw.
 */
/// How a script with these attributes runs, or `None` if it does not.
pub fn script_kind(ty: Option<&str>, language: Option<&str>) -> Option<ScriptKind> {
    let ty = match (ty, language) {
        (Some(t), _) => String::from(t.trim()),
        (None, Some(l)) if !l.is_empty() => alloc::format!("text/{}", l.trim()),
        (None, _) => String::new(),
    };
    if ty.is_empty() {
        return Some(ScriptKind::Classic);
    }
    if ty.eq_ignore_ascii_case("module") {
        return Some(ScriptKind::Module);
    }
    let essence = ty.split(';').next().unwrap_or("").trim();
    JS_TYPES.iter().any(|t| essence.eq_ignore_ascii_case(t)).then_some(ScriptKind::Classic)
}

/* The JavaScript MIME type essences of the MIME Sniffing standard. */
const JS_TYPES: [&str; 16] = [
    "application/ecmascript",
    "application/javascript",
    "application/x-ecmascript",
    "application/x-javascript",
    "text/ecmascript",
    "text/javascript",
    "text/javascript1.0",
    "text/javascript1.1",
    "text/javascript1.2",
    "text/javascript1.3",
    "text/javascript1.4",
    "text/javascript1.5",
    "text/jscript",
    "text/livescript",
    "text/x-ecmascript",
    "text/x-javascript",
];
