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

//! The last page loads this window made and how each ended, shown at
//! about:loads. The same lines go to the serial console, but only a build
//! that grants the browser Debug lets them through, and a person with a
//! page that will not load has the browser in front of them, not a serial
//! cable: here they read why without either.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

use spin::Mutex;

use super::wire::Wire;

/// Lines kept; the oldest goes first.
const KEPT: usize = 60;

static LINES: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// Keep `line` and write it to the serial console.
pub fn note<W: Wire>(w: &mut W, line: &str) {
    w.trace(line.as_bytes());
    keep(line);
}

/// Keep `line` only, for a caller with no wire to hand.
pub fn keep(line: &str) {
    let mut lines = LINES.lock();
    if lines.len() >= KEPT {
        lines.pop_front();
    }
    lines.push_back(String::from(line.trim_end()));
}

/// The kept lines, oldest first.
pub fn lines() -> Vec<String> {
    LINES.lock().iter().cloned().collect()
}

/// about:loads, as a page.
pub fn page() -> String {
    let mut html = String::from(
        "<html><head><title>Page loads</title><style>\
         body{background:#10151a;color:#dbe6ee}\
         pre{white-space:pre-wrap;color:#cfe3ee}\
         p{color:#7fa6b8}</style></head><body><h1>Page loads</h1>\
         <p>How each page this window loaded ended, newest last: the stage it \
         stopped in and why, how long it took, how much arrived, and the network \
         it went over. The same lines go to the serial console as [BROWSER].</p><pre>",
    );
    let lines = lines();
    if lines.is_empty() {
        html.push_str("No page has been loaded in this window yet.");
    }
    for line in lines {
        escape(&mut html, &line);
        html.push('\n');
    }
    html.push_str("</pre></body></html>");
    html
}

fn escape(out: &mut String, text: &str) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
}
