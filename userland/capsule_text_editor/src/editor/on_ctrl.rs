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

use nonos_app_skeleton::EventOutcome;

use super::ctrl_copy::ctrl_copy;
use super::ctrl_cut::ctrl_cut;
use super::ctrl_paste::ctrl_paste;
use super::layout::{MAX_SCALE, MIN_SCALE};
use super::mode::Mode;
use super::path_prompt;
use super::state::{PromptOp, State};

pub(super) fn on_ctrl(state: &mut State, code: u32, shift: bool) -> EventOutcome {
    match code {
        0x41 | 0x61 => select_all(state),
        0x43 | 0x63 => ctrl_copy(state),
        0x44 | 0x64 => line_edit(state, false),
        0x46 | 0x66 => open_find(state),
        // Ctrl+M switches between the two views.
        //
        // The view used to be decided once, from the file extension, and never
        // again: a .txt or a file with no extension opened as a document and
        // could not be shown as code, so its line numbers and syntax colouring
        // were unreachable no matter what was in it. The extension is a good
        // first guess and a bad final answer.
        0x4D | 0x6D => toggle_view(state),
        // Ctrl+G goes to a line by number, through the same prompt that opens
        // and saves. Free until now, and the one navigation a compiler message
        // sends you looking for.
        0x47 | 0x67 => path_prompt::start(state, PromptOp::Goto),
        0x48 | 0x68 if shift => replace_all(state),
        0x48 | 0x68 => open_replace(state),
        0x4B | 0x6B if shift => line_edit(state, true),
        0x45 | 0x65 => path_prompt::start(state, PromptOp::Export),
        0x4F | 0x6F => path_prompt::start(state, PromptOp::Open),
        // Ctrl+S writes straight to the file's own path; Ctrl+Shift+S (and a
        // document that never had a path) goes through the Save As prompt.
        0x53 | 0x73 if shift || state.path_len == 0 => path_prompt::start(state, PromptOp::Save),
        0x53 | 0x73 => super::ctrl_save::ctrl_save(state),
        0x56 | 0x76 => ctrl_paste(state),
        0x58 | 0x78 => ctrl_cut(state),
        0x2F => comment(state),
        // Ctrl+Z undoes; Ctrl+Y or Ctrl+Shift+Z redoes.
        0x5A | 0x7A => history(state, !shift),
        0x59 | 0x79 => history(state, false),
        // Ctrl+= / Ctrl++ zoom in, Ctrl+- / Ctrl+_ zoom out, Ctrl+0 reset.
        0x3D | 0x2B => zoom(state, 1),
        0x2D | 0x5F => zoom(state, -1),
        0x30 => zoom_reset(state),
        // Ctrl+Shift+B bolds the selection, Ctrl+I italicises it, Ctrl+U underlines it.
        0x42 | 0x62 if shift => run_toggle(state, 0),
        0x49 | 0x69 => run_toggle(state, 1),
        0x55 | 0x75 => run_toggle(state, 2),
        _ => EventOutcome::Idle,
    }
}

// Step the zoom level within bounds; a no-op at the limit stays idle so the
// screen is not repainted for nothing.
fn zoom(state: &mut State, delta: i32) -> EventOutcome {
    let next = (state.font_scale as i32 + delta).clamp(MIN_SCALE as i32, MAX_SCALE as i32) as u32;
    if next == state.font_scale {
        return EventOutcome::Idle;
    }
    state.font_scale = next;
    state.status = b"zoom";
    EventOutcome::Repaint
}

fn zoom_reset(state: &mut State) -> EventOutcome {
    if state.font_scale == 2 {
        return EventOutcome::Idle;
    }
    state.font_scale = 2;
    state.status = b"zoom reset";
    EventOutcome::Repaint
}

// A ribbon run-style toggle driven from the keyboard or a Format menu row, so
// the row and the shortcut cannot drift apart.
fn run_toggle(state: &mut State, t: usize) -> EventOutcome {
    state.apply_toggle(t);
    EventOutcome::Repaint
}

/// Flip between the document view and the code view.
///
/// Both are fully built; only the choice between them was missing. Scroll is
/// reset to the caret because the two views measure lines differently and a
/// scroll offset from one is meaningless in the other.
fn toggle_view(state: &mut State) -> EventOutcome {
    state.mode = match state.mode {
        Mode::Document => Mode::Code,
        Mode::Code => Mode::Document,
    };
    state.status = match state.mode {
        Mode::Code => b"code view",
        Mode::Document => b"page view",
    };
    let rows = state.visible_rows;
    super::follow_caret::follow_caret(state, rows);
    EventOutcome::Repaint
}

// Ctrl+/ comments or uncomments the caret line or the selected lines.
fn comment(state: &mut State) -> EventOutcome {
    if state.toggle_comment() {
        let rows = state.visible_rows;
        super::follow_caret::follow_caret(state, rows);
        state.status = b"edited";
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// Ctrl+D duplicates the caret line; Ctrl+Shift+K deletes it. Both follow the
// caret to where it lands and mark the buffer edited.
fn line_edit(state: &mut State, delete: bool) -> EventOutcome {
    let changed = if delete { state.delete_line() } else { state.duplicate_line() };
    if changed {
        let rows = state.visible_rows;
        super::follow_caret::follow_caret(state, rows);
        state.status = b"edited";
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// Ctrl-Z undoes, Ctrl-Y redoes; both collapse any selection and follow the
// caret to where the change happened.
fn history(state: &mut State, undo: bool) -> EventOutcome {
    let changed = if undo { state.undo() } else { state.redo() };
    if changed {
        state.clear_sel();
        let rows = state.visible_rows;
        super::follow_caret::follow_caret(state, rows);
        state.status = if undo { b"undo" } else { b"redo" };
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// Ctrl-A selects the whole buffer.
fn select_all(state: &mut State) -> EventOutcome {
    state.sel_anchor = Some(0);
    state.caret = state.len;
    EventOutcome::Repaint
}

// Ctrl-F opens the find bar, seeding it with the current selection.
fn open_find(state: &mut State) -> EventOutcome {
    state.find_active = true;
    if let Some((s, e)) = state.sel_range() {
        state.find_buf = alloc::string::String::from_utf8_lossy(&state.buf[s..e]).into_owned();
    }
    state.find_incremental();
    EventOutcome::Repaint
}

// Ctrl-H opens find with the replacement field focused.
fn open_replace(state: &mut State) -> EventOutcome {
    open_find(state);
    state.replace_active = true;
    EventOutcome::Repaint
}

// Ctrl-Shift-H rewrites every match in one pass and reports the count.
fn replace_all(state: &mut State) -> EventOutcome {
    let n = state.replace_all();
    state.status = if n > 0 { b"replaced all" } else { b"no matches" };
    let rows = state.visible_rows;
    super::follow_caret::follow_caret(state, rows);
    EventOutcome::Repaint
}
