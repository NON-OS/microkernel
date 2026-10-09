// NONOS Operating System (AGPL-3.0-or-later)
//! The interpreter's querySelectorAll, on the document and on an element,
//! hands back every match: it used to stop at the first 256 of the document
//! and, for an element, keep only those of them inside it.

use crate::browser::dom;
use crate::browser::js::ast::Expr;
use crate::browser::js::value::Value;
use crate::{run, set_n};

fn call(target: Expr, method: &str, arg: &str) -> Expr {
    let callee = Expr::Member(Box::new(target), method.into());
    Expr::Call(Box::new(callee), vec![Expr::Str(arg.into())])
}

fn length(e: Expr) -> Expr {
    Expr::Member(Box::new(e), "length".into())
}

fn page() -> dom::Dom {
    let mut html = String::from("<body><a>out</a><div id=box>");
    for i in 0..600 {
        html.push_str(&format!("<a href=/{i}>{i}</a>"));
    }
    html.push_str("</div></body>");
    dom::parse(html.as_bytes())
}

#[test]
fn document_query_all_returns_every_match() {
    let mut d = page();
    let doc = Expr::Ident("document".into());
    let n = run(&mut d, vec![set_n(length(call(doc, "querySelectorAll", "a")))]);
    assert!(matches!(n, Value::Num(v) if v == 601.0), "all 601 links, past the old 256 cap");
}

#[test]
fn element_query_all_walks_only_its_subtree_and_returns_every_match() {
    let mut d = page();
    let bx = call(Expr::Ident("document".into()), "getElementById", "box");
    let n = run(&mut d, vec![set_n(length(call(bx.clone(), "querySelectorAll", "a")))]);
    assert!(matches!(n, Value::Num(v) if v == 600.0), "the 600 links inside #box only");
    let last = call(bx, "querySelector", ":scope > a:nth-child(600)");
    assert!(matches!(run(&mut d, vec![set_n(last)]), Value::Node(_)), ":scope is the element");
}
