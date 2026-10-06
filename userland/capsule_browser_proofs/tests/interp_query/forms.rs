// NONOS Operating System (AGPL-3.0-or-later)
//! Every expression and statement form, in one program the interpreter runs
//! to a known answer, so all of the tree this crate compiles is exercised.

use crate::browser::js::ast::{ClassMethod, Expr as E, Stmt as S};

fn b(e: E) -> Box<E> {
    Box::new(e)
}

fn id(s: &str) -> E {
    E::Ident(s.into())
}

fn num(v: f64) -> E {
    E::Num(v)
}

fn op(o: &str, x: E, y: E) -> E {
    E::Binary(o.into(), b(x), b(y))
}

fn set(name: &str, e: E) -> S {
    S::Expr(E::Assign("=".into(), b(id(name)), b(e)))
}

fn var(name: &str, e: E) -> S {
    S::Var(vec![(name.into(), Some(e))])
}

/* s counts 1 (a call), up to 3 (a loop), +5 (an object index), +1 (a
 * ternary on null == undefined), -1 (a logical and a unary minus) and +1
 * (a caught throw): n ends as 9. */
pub fn program() -> Vec<S> {
    let plus = |x: E| set("s", op("+", id("s"), x));
    let ret = |e: E| vec![S::Return(Some(e))];
    let ctor =
        ClassMethod { name: "constructor".into(), params: vec![], body: vec![], is_async: false };
    let null_is_undef = op("==", E::Null, E::Undef);
    vec![
        var("s", num(0.0)),
        S::Func("add".into(), vec!["a".into(), "c".into()], ret(op("+", id("a"), id("c"))), false),
        S::If(
            E::Bool(true),
            vec![set("s", E::Call(b(id("add")), vec![id("s"), num(1.0)]))],
            vec![],
        ),
        S::While(op("<", id("s"), num(3.0)), vec![plus(num(1.0))]),
        S::For(
            Some(Box::new(var("i", num(0.0)))),
            Some(op("<", id("i"), num(2.0))),
            Some(E::Assign("=".into(), b(id("i")), b(op("+", id("i"), num(1.0))))),
            vec![S::Continue],
        ),
        S::ForOf("x".into(), E::Array(vec![num(1.0), num(2.0)]), vec![S::Break]),
        var("o", E::Object(vec![("k".into(), num(5.0))])),
        plus(E::Index(b(id("o")), b(E::Str("k".into())))),
        plus(E::Ternary(b(null_is_undef), b(num(1.0)), b(num(0.0)))),
        plus(E::Logical("&&".into(), b(E::Bool(true)), b(E::Unary("-".into(), b(num(1.0)))))),
        var("f", E::Func(vec![], ret(num(1.0)), false)),
        S::Class("C".into(), None, vec![ctor]),
        var("c", E::New(b(id("C")), vec![])),
        S::Try(vec![S::Throw(num(1.0))], Some((Some("e".into()), vec![plus(id("e"))])), None),
        var("r", E::Regex("a".into(), "g".into())),
        S::Block(vec![set("n", id("s"))]),
    ]
}
