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

//! The QuickJS engine a page's scripts run in.

use core::ffi::c_void;

use nonos_qjs::{Engine, Limits};

use super::dom::Dom;

/*
 * The capsule has a 96 MiB heap (main.rs), and laying out a large article
 * page peaks at 41 MiB of it on its own. A third of the heap for the page's
 * JavaScript leaves the DOM, the boxes, decoded images and fetch buffers
 * room beside it; a page that wants more gets an exception in its own code
 * rather than taking the browser down when the allocator runs dry.
 */
/// Heap the page's JavaScript may hold.
pub const SCRIPT_HEAP: usize = 32 * 1024 * 1024;

/*
 * The capsule's thread has a 2 MiB stack (USER_STACK_SIZE in the kernel).
 * Half of it below the frame a script is entered from is QuickJS's own
 * default; the other half is for the render loop above that frame and for
 * the callbacks a script makes back into the browser, the innerHTML parser
 * among them, which recurse on the same stack.
 */
/// Native stack the page's JavaScript may use.
pub const SCRIPT_STACK: usize = 1024 * 1024;

/*
 * Long enough for a framework's first render on a slow machine, short
 * enough that a page stuck in a loop gives the window back before the
 * reader decides the browser has hung.
 */
/// How long one entry into the page's code may run.
pub const SCRIPT_BUDGET_MS: u64 = 2_000;

/*
 * A page's own scripts, run once as it loads, build what the reader sees: a
 * framework's bundle is its render. One that took most of two seconds in
 * QEMU takes four times that on a laptop Celeron, and stopping it there left
 * a page that never drew. The window is held while they run, so this is a
 * bound for a page that never ends, not a pace for one that does.
 */
/// How long each of a page's own scripts may run as the page loads.
pub const PAGE_SCRIPT_BUDGET_MS: u64 = 10_000;

extern "C" fn uptime_ms() -> u64 {
    nonos_libc::mk_uptime_ms().max(0) as u64
}

/* The wall clock the fetch machine and the cookie jar read, so a page's
 * Date agrees with the expiry its cookies are judged at. A machine whose
 * clock was never set reads near zero here too, and the page is told so
 * rather than a date that was made up. */
extern "C" fn wall_ms() -> i64 {
    nonos_libc::mk_time_millis()
}

/// The page's engine with `dom` installed as its document, before any of
/// the page's own scripts have run: they run one by one, in document order,
/// as fetch::land::run_held reaches each. The DOM pointer lives in the
/// engine's context, so `dom` must outlive the returned engine.
pub fn page_engine(dom: &mut Dom) -> Option<Engine> {
    let limits = Limits {
        memory: SCRIPT_HEAP,
        stack: SCRIPT_STACK,
        budget_ms: SCRIPT_BUDGET_MS,
        clock: uptime_ms,
        wall: wall_ms,
    };
    let engine = Engine::with_limits(limits)?;
    /* The DOM sits in State::page_dom for the page's life, and the engine is
     * dropped before it is replaced (commit_doc), so the pointer stays good
     * for every call the page's code makes. */
    unsafe { engine.install_dom(dom as *mut Dom as *mut c_void) };
    Some(engine)
}
