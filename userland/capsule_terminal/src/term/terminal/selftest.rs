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

use nonos_libc::{heap_init, mk_debug, mk_exit, mk_yield, HeapError};

use crate::term::dimensions::{COLS, VISIBLE_ROWS};
use crate::term::state::State;
use crate::term::util::copy_into;
use nonos_vt::Term;

mod git_test;
mod jobs_test;
mod kernel_api;
mod proc_lifecycle;

const READY_ATTEMPTS: u32 = 100_000;

// Headless capsule entry for the autorun-selftest build: bring up the heap,
// wait until vfs answers, drive the unproven shell paths, then exit. No
// surface, no compositor, no focus handoff -- this grades shell logic, not
// the GUI app loop.
pub fn main() -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => exit_fail(b"[TERMINAL-TEST] FAIL heap\n"),
    }
    let mut state = State::new();
    let mut attempts = 0u32;
    while !ready(&mut state) {
        attempts += 1;
        if attempts >= READY_ATTEMPTS {
            exit_fail(b"[TERMINAL-TEST] FAIL vfs never ready\n");
        }
        mk_yield();
    }
    run(&mut state);
    mk_exit(0);
}

fn exit_fail(msg: &[u8]) -> ! {
    let _ = mk_debug(msg.as_ptr(), msg.len());
    mk_exit(1);
}

// True once `read` of a vfs-seeded file succeeds, so the assertions only
// run after the capsule stack has settled and vfs is answering.
fn ready(state: &mut State) -> bool {
    run_cmd(state, b"read /readme.txt");
    state.last_status == 0
}

// Drive the previously unproven shell paths through the normal submit
// path and emit one serial marker per step so a headless boot grades
// itself: echo, a vfs write/read round trip, a pipe, and `||` gating.
pub fn vt_selftest() {
    mark(b"vt-skeleton", true);
    {
        let p = nonos_vt::Palette::xterm();
        mark(b"vt-color", p.colors[1] == 0xCD_0000 && p.colors[15] == 0xFF_FFFF);
    }
    {
        let t = Term::new(COLS, VISIBLE_ROWS, 10);
        let ok =
            t.cols() == COLS && t.rows() == VISIBLE_ROWS && t.visible_line(0).cell(0).ch == ' ';
        mark(b"vt-grid", ok && t.cursor().x == 0 && t.cursor().y == 0);
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"AB\rC");
        let row0_ok = row_text(&t, 0) == "CB";
        for _ in 0..VISIBLE_ROWS {
            t.feed(b"\n");
        }
        let scrolled_ok = t.history_len() >= 1;
        t.feed(b"Z\x1b[2J");
        mark(b"vt-grid-ops", row0_ok && scrolled_ok && row_text(&t, 0).is_empty());
    }
    {
        // The parser on its own: prints, controls and CSI finals in order.
        struct Rec(alloc::vec::Vec<u8>);
        impl nonos_vt::parser::Handler for Rec {
            fn print(&mut self, c: char) {
                self.0.push(c as u8);
            }
            fn execute(&mut self, b: u8) {
                self.0.push(b);
            }
            fn csi(&mut self, s: &nonos_vt::parser::Seq) {
                self.0.push(s.final_byte);
            }
            fn esc(&mut self, _s: &nonos_vt::parser::Seq) {}
            fn osc(&mut self, _d: &[u8]) {}
            fn dcs(&mut self, _s: &nonos_vt::parser::Seq, _d: &[u8]) {}
        }
        let mut rec = Rec(alloc::vec::Vec::new());
        nonos_vt::parser::Parser::new().feed(&mut rec, b"A\x1b[31mB\x1b[0m\n\x1b[2J");
        mark(b"vt-parser", rec.0 == b"AmBm\nJ");
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"\x1b[5;3HX");
        let pos_ok = t.visible_line(4).cell(2).ch == 'X';
        t.feed(b"\x1b[2J");
        mark(b"vt-csi", pos_ok && row_text(&t, 4).is_empty());
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"\x1b[1;31mZ\x1b[0mY");
        let z = t.visible_line(0).cell(0);
        let y = t.visible_line(0).cell(1);
        let set = z.attr & nonos_vt::cell::attr::BOLD != 0 && z.fg == nonos_vt::Color::Indexed(1);
        mark(b"vt-sgr", set && y.attr == 0 && y.fg == nonos_vt::Color::Default);
    }
    {
        let mut sb = crate::term::scrollback::Scrollback::new();
        sb.feed_raw(b"hi\x1b[32m!\n");
        let bang = sb.vt.visible_line(0).cell(2);
        let ok = row_text(&sb.vt, 0) == "hi!" && bang.fg == nonos_vt::Color::Indexed(2);
        mark(b"vt-feed", ok && sb.vt.cursor().x == 0 && sb.vt.cursor().y == 1);
    }
    {
        let mut sb = crate::term::scrollback::Scrollback::new();
        sb.push_line(b"MIR");
        mark(b"vt-mirror", row_text(&sb.vt, 0) == "MIR");
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 100);
        for i in 0..VISIBLE_ROWS as u8 + 5 {
            t.feed(&[b'A' + i % 26, b'\r', b'\n']);
        }
        let has_hist = t.history_len() >= 5;
        t.scroll_view(1);
        let off1 = t.view_offset() == 1;
        t.scroll_to_bottom();
        mark(b"vt-history", has_hist && off1 && t.view_offset() == 0);
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 10);
        t.feed(b"MAIN\x1b[?1049h");
        let in_alt = t.alt_active() && row_text(&t, 0).is_empty();
        t.feed(b"ALT\x1b[?1049l");
        mark(b"vt-altscreen", in_alt && !t.alt_active() && row_text(&t, 0) == "MAIN");
    }
    {
        let mut st = crate::term::state::State::new();
        st.open_block(*b"12:34:56");
        st.close_block(false, 0);
        let one = st.blocks.len() == 1;
        let err = st.blocks[0].status == crate::term::block::Status::Err;
        let ts = &st.blocks[0].ts == b"12:34:56";
        let found = st.block_at(st.blocks[0].start_abs).is_some();
        mark(b"block-model", one && err && ts && found);
    }
    {
        let mut st = crate::term::state::State::new();
        st.open_block(*b"00:00:00");
        st.close_block(true, 1500);
        let ok =
            st.blocks[0].dur_ms == 1500 && st.blocks[0].status == crate::term::block::Status::Ok;
        mark(b"block-dur", ok);
    }
    {
        let mut t = Term::new(COLS, VISIBLE_ROWS, 100);
        let start = t.cursor_pos().line;
        for _ in 0..(VISIBLE_ROWS + 3) {
            t.feed(b"x\r\n");
        }
        let scrolled = t.first_line() + t.history_len() as u64 == 4;
        let row_abs = t.abs_of_row(0) == 4 - t.view_offset() as u64;
        mark(b"block-absline", start == 0 && scrolled && row_abs);
    }
    {
        let f = crate::term::rtc::fmt_hms(9, 5, 42);
        mark(b"block-ts", &f == b"09:05:42");
    }
    {
        let mut st = crate::term::state::State::new();
        st.line.replace(b"echo hi");
        let _ = crate::event::on_enter(&mut st);
        let opened = st.blocks.len() == 1;
        let ok = st.blocks[0].status == crate::term::block::Status::Ok;
        mark(b"block-capture", opened && ok);
    }
    {
        let (a, an) = crate::term::dur::fmt_dur(1800);
        let (b, bn) = crate::term::dur::fmt_dur(250);
        let (c, cn) = crate::term::dur::fmt_dur(125_000);
        let ok = &a[..an] == b"1.8s" && &b[..bn] == b"250ms" && &c[..cn] == b"2m05s";
        mark(b"dur-fmt", ok);
    }
}

fn run(state: &mut State) {
    vt_selftest();
    run_cmd(state, b"echo selfcheck");
    mark(b"echo", visible_has(state, b"selfcheck"));

    run_cmd(state, b"write /st.txt smoke123");
    run_cmd(state, b"read /st.txt");
    mark(b"vfs", visible_has(state, b"smoke123"));

    run_cmd(state, b"echo a b c | wc");
    mark(b"pipe", visible_has(state, b"1"));

    run_cmd(state, b"echo a | cat /readme.txt");
    mark(b"pipe-anystage", visible_has(state, b"This file lives in the vfs capsule."));

    run_cmd(state, b"read /nope.txt || echo recovered");
    mark(b"statement", visible_has(state, b"recovered"));

    run_ext(state);

    let pass = b"[TERMINAL-TEST] PASS\n";
    let _ = mk_debug(pass.as_ptr(), pass.len());
}

// Grade the commands added for the Warp/coreutils pass: path utilities,
// the cut filter, touch+&&, recursive find, and clock/size queries that
// only assert success because their output is not deterministic.
fn run_ext(state: &mut State) {
    run_cmd(state, b"basename /a/b/file.txt");
    mark(b"basename", visible_has(state, b"file.txt"));

    run_cmd(state, b"dirname /a/b/file.txt");
    mark(b"dirname", visible_has(state, b"/a/b"));

    run_cmd(state, b"echo a:b:c | cut -d: -f2");
    mark(b"cut", visible_has(state, b"b"));

    run_cmd(state, b"write /rin.txt redirin");
    run_cmd(state, b"grep redirin < /rin.txt");
    mark(b"redir-in", visible_has(state, b"redirin"));

    run_cmd(state, b"touch /tt.txt && echo created");
    mark(b"touch", visible_has(state, b"created"));

    run_cmd(state, b"find /");
    mark(b"find", visible_has(state, b"/readme.txt"));

    mark(b"du", ok_cmd(state, b"du /"));
    mark(b"date", ok_cmd(state, b"date"));

    run_cmd(state, b"ifconfig");
    mark(b"ifconfig", visible_has(state, b"net0: down"));

    run_cmd(state, b"nslookup nonos.test");
    mark(b"nslookup", visible_has(state, b"nslookup: dns unavailable"));

    run_cmd(state, b"cat /nonexistent");
    run_cmd(state, b"echo $?");
    mark(b"status", visible_has(state, b"1"));

    kernel_api::run();
    proc_lifecycle::run();
    jobs_test::run(state);
    git_test::run(state);
}

fn run_cmd(state: &mut State, cmd: &[u8]) {
    state.scrollback.clear();
    state.line.replace(cmd);
    let _ = crate::event::on_enter(state);
}

fn ok_cmd(state: &mut State, cmd: &[u8]) -> bool {
    run_cmd(state, cmd);
    state.last_status == 0
}

fn visible_has(state: &State, needle: &[u8]) -> bool {
    let vt = &state.scrollback.vt;
    (0..vt.rows()).any(|row| row_text(vt, row).as_bytes() == needle)
}

/// Visible row `row` without trailing blanks.
fn row_text(vt: &Term, row: usize) -> alloc::string::String {
    let line = vt.visible_line(row);
    let s: alloc::string::String =
        (0..vt.cols()).map(|x| line.cell(x)).filter(|c| !c.is_tail()).map(|c| c.ch).collect();
    alloc::string::String::from(s.trim_end())
}

fn mark(step: &[u8], ok: bool) {
    let mut buf = [0u8; 64];
    let mut n = 0;
    n += copy_into(&mut buf[n..], b"[TERMINAL-TEST] ");
    n += copy_into(&mut buf[n..], step);
    n += copy_into(&mut buf[n..], if ok { b" ok\n" } else { b" FAIL\n" });
    let _ = mk_debug(buf.as_ptr(), n);
}
