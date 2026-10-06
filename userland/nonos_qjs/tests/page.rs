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

//! The real engine with the DOM bindings installed over a host with no
//! tree: the page's load events, document.cookie, and the time budget on
//! timers and event listeners.
//!
//! Run with `cargo test --release --features hosted`.

use core::ffi::c_void;
use core::ptr::null_mut;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Instant;

use nonos_qjs::{Dialog, Engine, Key, Limits, Press, Stop, MOD_ALT, MOD_CTRL, MOD_SHIFT};

static START: OnceLock<Instant> = OnceLock::new();
static COOKIES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static STYLE_SETS: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
static WIDTH: AtomicU32 = AtomicU32::new(1024);
static VALUES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static SCROLL: AtomicU32 = AtomicU32::new(0);
static INTO_VIEW: Mutex<Vec<(i32, i32)>> = Mutex::new(Vec::new());

extern "C" fn clock() -> u64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as u64
}

extern "C" fn wall() -> i64 {
    1_767_225_600_000 + clock() as i64
}

extern "C" {
    fn malloc(n: usize) -> *mut u8;
}

/// A C string the bindings free, as every string a host returns is.
fn owned(s: &str) -> *mut u8 {
    unsafe {
        let p = malloc(s.len() + 1);
        core::ptr::copy_nonoverlapping(s.as_ptr(), p, s.len());
        *p.add(s.len()) = 0;
        p
    }
}

/* The host callbacks. The tree is a lone root, node 0, with nothing in it;
 * what these tests drive is the engine around it. */
#[no_mangle]
pub extern "C" fn njs_dom_cookie_get(_h: *mut c_void) -> *mut u8 {
    owned("theme=dark; lang=en")
}
#[no_mangle]
pub extern "C" fn njs_dom_cookie_set(_h: *mut c_void, line: *const u8) {
    let s = unsafe { core::ffi::CStr::from_ptr(line as *const core::ffi::c_char) };
    COOKIES.lock().expect("lock").push(s.to_string_lossy().into_owned());
}
fn text(p: *const u8) -> String {
    let s = unsafe { core::ffi::CStr::from_ptr(p as *const core::ffi::c_char) };
    s.to_string_lossy().into_owned()
}
/* The body is node 0. Its inline style gives display:none, and every write
 * to it is kept, so the style proxy is seen to read and write through the
 * host rather than answer for it. */
#[no_mangle]
pub extern "C" fn njs_dom_body(_h: *mut c_void) -> i32 {
    0
}
#[no_mangle]
pub extern "C" fn njs_dom_get_style(_h: *mut c_void, node: i32, prop: *const u8) -> *mut u8 {
    match (node, text(prop).as_str()) {
        (0, "display") => owned("none"),
        (0, "cssText") => owned("display: none;"),
        _ => owned(""),
    }
}
#[no_mangle]
pub extern "C" fn njs_dom_set_style(_h: *mut c_void, node: i32, prop: *const u8, val: *const u8) {
    assert_eq!(node, 0, "the style written is the body's");
    STYLE_SETS.lock().expect("lock").push((text(prop), text(val)));
}
/* How far the host's page is scrolled. */
#[no_mangle]
pub extern "C" fn njs_dom_scroll_y(_h: *mut c_void) -> i32 {
    SCROLL.load(Ordering::SeqCst) as i32
}
/* A script's scrollTo: the host holds the page to 0..=1000 and answers
 * where it is. */
#[no_mangle]
pub extern "C" fn njs_dom_scroll_to(_h: *mut c_void, y: i32) -> i32 {
    let y = y.clamp(0, 1000);
    SCROLL.store(y as u32, Ordering::SeqCst);
    y
}
/* scrollIntoView: which node and which block the host was asked for. */
#[no_mangle]
pub extern "C" fn njs_dom_scroll_into_view(_h: *mut c_void, node: i32, block: i32) -> i32 {
    INTO_VIEW.lock().expect("lock").push((node, block));
    SCROLL.load(Ordering::SeqCst) as i32
}
/* The control's value comes from the host, not from an attribute. */
#[no_mangle]
pub extern "C" fn njs_dom_get_value(_h: *mut c_void, node: i32) -> *mut u8 {
    owned(if node == 0 { "top" } else { "" })
}
#[no_mangle]
pub extern "C" fn njs_dom_set_value(_h: *mut c_void, node: i32, v: *const u8) {
    assert_eq!(node, 0, "the value written is the body's");
    VALUES.lock().expect("lock").push(text(v));
}
/* The host's cascade: the body is a block with a red background, and the
 * name asked is echoed for anything else, as the host saw it. */
#[no_mangle]
pub extern "C" fn njs_dom_computed(_h: *mut c_void, node: i32, prop: *const u8) -> *mut u8 {
    assert_eq!(node, 0, "the style read is the body's");
    match text(prop).as_str() {
        "display" => owned("block"),
        "background-color" | "backgroundColor" => owned("rgb(255, 0, 0)"),
        other => owned(&format!("?{other}")),
    }
}
/* The reader's history: four entries, the second of them shown. */
#[no_mangle]
pub extern "C" fn njs_dom_history(_h: *mut c_void, which: i32) -> i32 {
    if which == 0 {
        4
    } else {
        1
    }
}
/* The host's media judge, for the one query the test asks. */
#[no_mangle]
pub extern "C" fn njs_dom_media_matches(_h: *mut c_void, query: *const u8) -> i32 {
    assert_eq!(text(query), "(min-width: 768px)");
    (WIDTH.load(Ordering::SeqCst) >= 768) as i32
}
#[no_mangle]
pub extern "C" fn njs_dom_base_url(_h: *mut c_void) -> *mut u8 {
    owned("https://page.example/a/b")
}
#[no_mangle]
pub extern "C" fn njs_dom_resolve(_h: *mut c_void, _r: *const u8) -> *mut u8 {
    null_mut()
}
macro_rules! none_str {
    ($($name:ident($($a:ty),*);)*) => {$(
        #[no_mangle]
        pub extern "C" fn $name(_h: *mut c_void $(, _: $a)*) -> *mut u8 { null_mut() }
    )*};
}
macro_rules! int {
    ($v:expr; $($name:ident($($a:ty),*);)*) => {$(
        #[no_mangle]
        pub extern "C" fn $name(_h: *mut c_void $(, _: $a)*) -> i32 { $v }
    )*};
}
macro_rules! nothing {
    ($($name:ident($($a:ty),*);)*) => {$(
        #[no_mangle]
        pub extern "C" fn $name(_h: *mut c_void $(, _: $a)*) {}
    )*};
}
none_str! {
    njs_dom_attr_name_at(i32, i32);
    njs_dom_get_attr(i32, *const u8);
    njs_dom_get_inner_html(i32);
    njs_dom_get_outer_html(i32);
    njs_dom_get_tag(i32);
    njs_dom_get_text(i32);
}
int! { -1;
    njs_dom_child_at(i32, i32);
    njs_dom_clone_node(i32, i32);
    njs_dom_closest(i32, *const u8);
    njs_dom_create_element(*const u8);
    njs_dom_create_fragment();
    njs_dom_create_text(*const u8);
    njs_dom_get_by_id(*const u8);
    njs_dom_last_child(i32);
    njs_dom_next_sibling(i32);
    njs_dom_parent(i32);
    njs_dom_prev_sibling(i32);
    njs_dom_query(*const u8);
    njs_dom_replace_child(i32, i32, i32);
}
int! { 0;
    njs_dom_attr_count(i32);
    njs_dom_box(i32, i32);
    njs_dom_child_count(i32);
    njs_dom_contains(i32, i32);
    njs_dom_has_attr(i32, *const u8);
    njs_dom_matches(i32, *const u8);
    njs_dom_node_kind(i32);
    njs_dom_query_all(*const u8, *mut i32, i32);
}
nothing! {
    njs_dom_append(i32, i32);
    njs_dom_insert_before(i32, i32, i32);
    njs_dom_remove_attr(i32, *const u8);
    njs_dom_remove_child(i32, i32);
    njs_dom_set_attr(i32, *const u8, *const u8);
    njs_dom_set_inner_html(i32, *const u8);
    njs_dom_set_text(i32, *const u8);
}

const BUDGET_MS: u64 = 300;

fn page() -> Engine {
    let limits =
        Limits { memory: 32 * 1024 * 1024, stack: 1024 * 1024, budget_ms: BUDGET_MS, clock, wall };
    let e = Engine::with_limits(limits).expect("an engine");
    /* No callback here reads the host, so any address will do. */
    let host = Box::leak(Box::new(0u8));
    unsafe { e.install_dom(host as *mut u8 as *mut c_void) };
    e
}

fn timed<T>(f: impl FnOnce() -> T) -> (T, u64) {
    let at = Instant::now();
    let out = f();
    (out, at.elapsed().as_millis() as u64)
}

/* One test: the stop flag is process wide, as it is in the capsule. */
#[test]
fn a_page_loads_keeps_its_cookies_and_survives_runaway_code() {
    let e = page();

    // The load events, in order, with readyState as each listener sees it.
    e.eval(
        "var seen=[];\
         document.addEventListener('DOMContentLoaded',function(){seen.push('dcl:'+document.readyState)});\
         window.addEventListener('load',function(){seen.push('load:'+document.readyState)});\
         seen.push('script:'+document.readyState);",
    );
    assert!(e.advance_ready_state(false));
    assert!(e.advance_ready_state(true));
    assert!(!e.advance_ready_state(true), "load fires once");
    assert_eq!(e.eval("seen.join()"), "script:loading,dcl:interactive,load:complete");

    // document.cookie goes to the host and back.
    assert_eq!(e.eval("document.cookie"), "theme=dark; lang=en");
    e.eval("document.cookie='seen=1; path=/'");
    assert_eq!(COOKIES.lock().expect("lock").as_slice(), ["seen=1; path=/"]);

    // el.style reads what the host's inline style gives, and writes go to
    // it, as do the CSSStyleDeclaration methods.
    assert_eq!(e.eval("document.body.style.display"), "none");
    assert_eq!(e.eval("document.body.style.color"), "");
    assert_eq!(e.eval("document.body.style.cssText"), "display: none;");
    assert_eq!(e.eval("document.body.style.getPropertyValue('display')"), "none");
    assert_eq!(
        e.eval(
            "var s=document.body.style; s.display = s.display === 'none' ? 'block' : 'none';\
             s.setProperty('margin-top','4px'); s.backgroundColor=null;\
             s.removeProperty('display')"
        ),
        "none",
        "removeProperty answers the value it removed"
    );
    let sets = STYLE_SETS.lock().expect("lock").clone();
    let want =
        [("display", "block"), ("margin-top", "4px"), ("backgroundColor", ""), ("display", "")];
    let want: Vec<(String, String)> =
        want.iter().map(|(p, v)| (p.to_string(), v.to_string())).collect();
    assert_eq!(sets, want);

    // location.href = url and location = url are navigations; a hash is not.
    assert_eq!(e.eval("location.pathname"), "/a/b");
    e.eval("location.href = 'https://page.example/next'");
    assert_eq!(e.take_navigation().as_deref(), Some("https://page.example/next"));
    e.eval("window.location = 'https://page.example/other'");
    assert_eq!(e.take_navigation().as_deref(), Some("https://page.example/other"));
    e.eval("location.hash = 'part'");
    assert_eq!(e.take_navigation(), None, "a new hash stays on the page");
    assert_eq!(e.eval("document.URL"), "https://page.example/a/b#part");

    // history.back() and go(n) are parked for the browser, read once.
    assert_eq!(e.take_history_step(), None);
    e.eval("history.back()");
    assert_eq!(e.take_history_step(), Some(-1));
    assert_eq!(e.take_history_step(), None, "a step is taken once");
    e.eval("history.forward(); history.go(-2)");
    assert_eq!(e.take_history_step(), Some(-2), "the last step asked wins");

    // history.length is the reader's history as the host keeps it (four
    // entries, the second shown); a push drops what was ahead of it.
    assert_eq!(e.eval("history.length"), "4");
    e.eval("history.pushState({}, '')");
    assert_eq!(e.eval("history.length"), "3", "two kept, and the push");
    e.eval("history.pushState({}, ''); history.replaceState({}, '')");
    assert_eq!(e.eval("history.length"), "4");

    // el.value is the host's reading of the control, both ways.
    assert_eq!(e.eval("document.body.value"), "top");
    e.eval("document.body.value = 'new'");
    assert_eq!(VALUES.lock().expect("lock").as_slice(), ["new"]);

    // matchMedia asks the host, every time it is read; a resize tells the
    // lists that are listened to and changed, then window.
    assert_eq!(e.eval("matchMedia('(min-width: 768px)').matches"), "true");
    e.eval(
        "var mq=matchMedia('(min-width: 768px)'), heard=[];\
         mq.addEventListener('change', function(ev){heard.push('change:'+ev.matches)});\
         mq.onchange=function(ev){heard.push('onchange:'+ev.media)};\
         window.addEventListener('resize', function(){heard.push('resize')});",
    );
    WIDTH.store(600, Ordering::SeqCst);
    assert_eq!(e.eval("mq.matches"), "false", "matches is read afresh");
    e.viewport_changed();
    assert_eq!(e.eval("heard.join()"), "change:false,onchange:(min-width: 768px),resize");
    e.viewport_changed();
    assert_eq!(
        e.eval("heard.join()"),
        "change:false,onchange:(min-width: 768px),resize,resize",
        "an answer that did not change is not reported again"
    );

    // An interval that never returns is stopped, and then dropped.
    e.eval("var spins=0; setInterval(function(){spins++; while(true){}}, 10);");
    let (ran, took) = timed(|| e.flush_timers(100));
    assert!(ran > 0, "the flush reports that something ran");
    assert!((BUDGET_MS..BUDGET_MS * 4).contains(&took), "stopped near its budget: {took} ms");
    assert_eq!(e.take_stop(), Some(Stop::Time));
    let (_, again) = timed(|| e.flush_timers(1_000));
    assert!(again < BUDGET_MS / 2, "the runaway interval is not run again: {again} ms");
    assert_eq!(e.take_stop(), None);
    assert_eq!(e.eval("spins"), "1");

    // A well-behaved timer still runs afterwards.
    e.eval("var later=0; setTimeout(function(){later=1}, 5);");
    e.flush_timers(2_000);
    assert_eq!(e.eval("later"), "1");

    // A UI event bubbles on past the document to document.on<type> and to
    // window, with the time it happened on it.
    e.eval(
        "var order=[], stamp=-1;\
         document.addEventListener('click', function(ev){order.push('doc'); stamp=ev.timeStamp});\
         document.onclick=function(){order.push('doc.onclick')};\
         window.addEventListener('click', function(ev){order.push('window:'+(ev.currentTarget===window))});\
         window.onclick=function(){order.push('window.onclick')};",
    );
    assert_eq!(e.dispatch_event(0, "click"), 4, "every listener counts as fired");
    assert_eq!(e.eval("order.join()"), "doc,doc.onclick,window:true,window.onclick");
    assert_eq!(e.eval("stamp > 0 && stamp <= performance.now()"), "true", "a real timeStamp");
    e.eval(
        "order=[]; document.addEventListener('input', function(ev){ev.stopPropagation()});\
         window.addEventListener('input', function(){order.push('window')});",
    );
    e.dispatch_event(0, "input");
    assert_eq!(e.eval("order.join()"), "", "a stopped event goes no further");
    assert_eq!(e.eval("new Event('x').timeStamp > 0"), "true");

    // window.scrollY is where the reader scrolled to, a rect is measured
    // from the window's top, and a scroll is heard on document and window.
    SCROLL.store(120, Ordering::SeqCst);
    assert_eq!(e.eval("scrollY + ',' + pageYOffset + ',' + scrollX"), "120,120,0");
    assert_eq!(e.eval("document.body.getBoundingClientRect().top"), "-120");
    e.eval(
        "var scrolled=[]; document.onscroll=function(ev){scrolled.push('doc:'+(ev.target===document))};\
         window.addEventListener('scroll', function(){scrolled.push('window:'+scrollY)});",
    );
    assert_eq!(e.dispatch_event(-1, "scroll"), 2);
    assert_eq!(e.eval("scrolled.join()"), "doc:true,window:120");

    // scrollTo, scroll and scrollBy move the page through the host, which
    // holds it to what can scroll, and scrollY reads the new place at once.
    assert_eq!(e.eval("scrollTo(0, 300); scrollY"), "300");
    assert_eq!(e.eval("scrollBy(0, -100); scrollY"), "200");
    assert_eq!(e.eval("scrollBy({top: 50, behavior: 'smooth'}); scrollY"), "250");
    assert_eq!(e.eval("scroll({top: 5000}); scrollY"), "1000", "held to the page");
    assert_eq!(e.eval("scrollTo(-20, -20); pageYOffset"), "0");
    assert_eq!(e.eval("scrollTo(0, 40); scrollTo({left: 9}); scrollTo(7); scrollY"), "40");
    assert_eq!(e.eval("scrollTo(0, NaN); scrollY"), "0");
    // scrollIntoView asks the host for the element and where it goes.
    e.eval(
        "var b=document.body; b.scrollIntoView(); b.scrollIntoView(false);\
         b.scrollIntoView({block:'center'}); b.scrollIntoView({block:'nearest'});\
         b.scrollIntoView({behavior:'smooth'}); b.scrollIntoView(true);",
    );
    assert_eq!(
        INTO_VIEW.lock().expect("lock").as_slice(),
        [(0, 0), (0, 2), (0, 1), (0, 3), (0, 0), (0, 0)]
    );

    // A key reaches the element, the document and window with what it is,
    // and a listener that cancels it is heard.
    e.eval(
        "var keys=[]; document.body.addEventListener('keydown', function(ev){\
         keys.push(ev.type+':'+ev.key+':'+ev.code+':'+ev.keyCode+':'+ev.shiftKey+ev.ctrlKey);\
         if(ev.key==='Escape')ev.preventDefault();});\
         window.addEventListener('keydown', function(ev){keys.push('window:'+ev.key)});\
         document.addEventListener('keyup', function(ev){keys.push('up:'+ev.key)});",
    );
    let esc = Key { key: "Escape", code: "Escape", key_code: 27, mods: 0 };
    assert_eq!(e.dispatch_key(0, "keydown", &esc), 2);
    assert!(e.default_prevented(), "the page kept Esc for itself");
    let a = Key { key: "A", code: "KeyA", key_code: 65, mods: MOD_SHIFT | MOD_CTRL };
    e.dispatch_key(0, "keydown", &a);
    assert!(!e.default_prevented());
    e.dispatch_key(0, "keyup", &a);
    assert_eq!(
        e.eval("keys.join()"),
        "keydown:Escape:Escape:27:falsefalse,window:Escape,\
         keydown:A:KeyA:65:truetrue,window:A,up:A"
    );

    // A press carries where it was: in the window, and on the page below
    // what is scrolled away.
    SCROLL.store(50, Ordering::SeqCst);
    e.eval(
        "var at=[]; document.body.onmousedown=function(ev){\
         at.push(ev.type+':'+ev.clientX+','+ev.clientY+':'+ev.pageY+':'+ev.button+ev.buttons)};\
         document.addEventListener('pointerdown', function(ev){\
         at.push(ev.type+':'+ev.pointerType+':'+ev.isPrimary+':'+ev.offsetY)});\
         window.addEventListener('click', function(ev){at.push('click:'+ev.clientX+':'+ev.altKey)});",
    );
    let down = Press { x: 30, y: 40, button: 0, buttons: 1, mods: 0 };
    e.dispatch_press(0, "pointerdown", &down);
    e.dispatch_press(0, "mousedown", &down);
    let click = Press { buttons: 0, mods: MOD_ALT, ..down };
    e.dispatch_press(0, "click", &click);
    assert_eq!(
        e.eval("at.join()"),
        "pointerdown:mouse:true:90,mousedown:30,40:90:01,click:30:true"
    );

    // getComputedStyle asks the host for the element's style, in either
    // spelling, and answers empty for what is not an element.
    assert_eq!(e.eval("getComputedStyle(document.body).display"), "block");
    assert_eq!(
        e.eval("getComputedStyle(document.body).getPropertyValue('background-color')"),
        "rgb(255, 0, 0)"
    );
    assert_eq!(e.eval("getComputedStyle(document.body).backgroundColor"), "rgb(255, 0, 0)");
    assert_eq!(e.eval("getComputedStyle(document.body)['--gap']"), "?--gap");
    assert_eq!(e.eval("getComputedStyle(null).color + '|' + getComputedStyle({}).display"), "|");
    assert_eq!(e.eval("typeof getComputedStyle(document.body).getPropertyValue"), "function");

    // alert, confirm and prompt answer at once, never yes, and leave what
    // was asked for the browser to say, read once.
    assert_eq!(e.take_dialog(), None);
    assert_eq!(e.eval("String(alert('Saved'))"), "undefined");
    let said = e.take_dialog().expect("the alert");
    assert_eq!((said.kind, said.text.as_str(), said.count), (Dialog::Alert, "Saved", 1));
    assert_eq!(e.take_dialog(), None, "a dialog is said once");
    assert_eq!(e.eval("confirm('Leave this page?')"), "false");
    assert_eq!(e.eval("String(prompt('Your name?', 'Ann'))"), "null");
    let said = e.take_dialog().expect("the prompt");
    assert_eq!((said.kind, said.text.as_str(), said.count), (Dialog::Prompt, "Your name?", 2));
    assert_eq!(e.eval("confirm(); 'went on'"), "went on");
    assert_eq!(e.take_dialog().map(|a| a.text), Some(String::new()));
    // A long message is cut to a whole character.
    e.eval("alert('\u{e9}'.repeat(300))");
    let said = e.take_dialog().expect("the long alert");
    assert_eq!(said.text, "\u{e9}".repeat(255));

    // A listener that never returns is stopped, and the page goes on.
    e.eval("document.addEventListener('click', function(){ while(true){} });");
    let (_, took) = timed(|| e.dispatch_event(0, "click"));
    assert!((BUDGET_MS..BUDGET_MS * 4).contains(&took), "the listener stopped: {took} ms");
    assert_eq!(e.take_stop(), Some(Stop::Time));
    assert_eq!(e.eval("'usable'"), "usable");

    // A promise chain that never ends is stopped too.
    let (out, took) = timed(|| e.eval("(function f(){ Promise.resolve().then(f); })(); 'queued'"));
    assert_eq!(out, "queued");
    assert!((BUDGET_MS..BUDGET_MS * 4).contains(&took), "the job queue stopped: {took} ms");
    assert_eq!(e.take_stop(), Some(Stop::Time));
}
