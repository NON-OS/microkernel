// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's script interpreter on the proof crate's DOM and selector
//! engine: its query methods, every syntax form it evaluates, and the path a
//! finished request takes back into a script.
extern crate alloc;

mod browser;
mod forms;
mod query;

use std::rc::Rc;

use browser::dom;
use browser::js::ast::{Expr, Stmt};
use browser::js::interp::{deliver_net, pump_timers};
use browser::js::value::{FuncData, Value};
use browser::js::world::{Timer, World};

/* A function value with no parameters (or one, `r`) over `body`. */
fn func(world: &World, params: &[&str], body: Vec<Stmt>) -> Value {
    let params = params.iter().map(|p| p.to_string()).collect();
    let env = world.env.clone();
    Value::Func(Rc::new(FuncData { params, body, env, is_async: false }))
}

/* Run `body` once through the timer pump and read the global n back. */
pub fn run(d: &mut dom::Dom, body: Vec<Stmt>) -> Value {
    let mut world = World::empty();
    world.env.define("n", Value::Undef);
    let cb = func(&world, &[], body);
    world.timers.push(Timer { cb, left: 1, every: None });
    pump_timers(d, &mut world);
    world.env.get("n").unwrap_or(Value::Undef)
}

/* `n = expr` as a statement. */
pub fn set_n(expr: Expr) -> Stmt {
    let n = Box::new(Expr::Ident("n".into()));
    Stmt::Expr(Expr::Assign("=".into(), n, Box::new(expr)))
}

#[test]
fn every_syntax_form_evaluates() {
    let mut d = dom::parse(b"<p>x</p>");
    assert!(matches!(run(&mut d, forms::program()), Value::Num(v) if v == 9.0));
}

#[test]
fn a_finished_request_reaches_its_callback() {
    let mut d = dom::parse(b"<p>x</p>");
    let mut world = World::empty();
    world.env.define("n", Value::Undef);
    let status = Expr::Member(Box::new(Expr::Ident("r".into())), "status".into());
    let cb = func(&world, &["r"], vec![set_n(status)]);
    deliver_net(&mut d, &mut world, cb, 201, "{\"a\":1}".into());
    assert!(matches!(world.env.get("n"), Some(Value::Num(v)) if v == 201.0));
    assert!(world.net_active.is_none(), "no request is left on the wire");
}
