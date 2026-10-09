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

//! The steps an open is made of, said while it runs and timed when it ends.
//!
//! An open is a few file writes and a few hundred milliseconds of key
//! derivation, even on an emulated machine. One that runs for minutes is
//! stuck, not slow, and the step it is stuck at is what the wallet shows,
//! so a boot names it instead of a clock that only counts.

use std::sync::Mutex;

/// The step running now, the uptime it began, and each finished step with
/// the milliseconds it took.
struct Steps {
    now: Option<(&'static str, u64)>,
    took: Vec<(&'static str, u64)>,
}

static STEPS: Mutex<Steps> = Mutex::new(Steps { now: None, took: Vec::new() });

/// The private address of the open under way, derived from its words or key before the store
/// is made, so the wallet shows it while the rest runs. Public by nature: it is the address
/// others pay to.
static PREVIEW: Mutex<Option<String>> = Mutex::new(None);

/// Keep the private address the open under way will answer.
pub fn preview(address: Option<String>) {
    if let Ok(mut p) = PREVIEW.lock() {
        *p = address;
    }
}

/// The private address of the open under way, once derived.
pub fn previewed() -> Option<String> {
    PREVIEW.lock().ok().and_then(|p| p.clone())
}

/// One line on the serial console: what the service is doing, and when.
/// Names and times only, never a field of a request: no words, key or
/// address reaches the console.
pub fn serial(what: &str) {
    let line = format!("[shield {} ms] {what}\n", uptime());
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

fn uptime() -> u64 {
    nonos_libc::mk_uptime_ms().max(0) as u64
}

/// A new open: nothing done yet.
pub fn begin() {
    preview(None);
    if let Ok(mut s) = STEPS.lock() {
        s.now = None;
        s.took.clear();
    }
}

/// The step `name` begins; the one before it is over.
pub fn step(name: &'static str) {
    let at = uptime();
    if let Ok(mut s) = STEPS.lock() {
        if let Some((last, since)) = s.now.take() {
            s.took.push((last, at.saturating_sub(since)));
        }
        s.now = Some((name, at));
    }
    serial(&format!("open: {name}"));
}

/// The last step is over: every step with its milliseconds, as one value.
pub fn end() -> String {
    step("done");
    let Ok(mut s) = STEPS.lock() else { return String::new() };
    s.now = None;
    s.took.iter().map(|(n, ms)| format!("{n} {ms} ms")).collect::<Vec<_>>().join(", ")
}

/// The step running now, and for how long, while an open runs.
pub fn running() -> Option<(&'static str, u64)> {
    let s = STEPS.lock().ok()?;
    s.now.map(|(name, since)| (name, uptime().saturating_sub(since)))
}
