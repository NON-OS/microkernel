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

//! Route input through the shell in priority order: the sidebar's name entry
//! takes the keyboard while it is open, an open context menu takes the next
//! click, then the activity bar, the explorer rows (left click opens, right
//! click brings up the menu), the tab strip, and finally the active document.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ENTER, MOD_CTRL, MOD_SHIFT};
use nonos_libc::mk_getpid;

use super::app::Editor;
use super::state::PromptOp;
use super::event::on_event;
use super::shell::pane_rect;

impl Editor {
    pub(super) fn handle_editor_event(&mut self, event: InputEvent) -> EventOutcome {
        if self.owner_pid == 0 {
            self.owner_pid = mk_getpid();
        }

        if self.panel.is_some() && event.is_key_down() {
            self.panel = None;
            return EventOutcome::Repaint;
        }

        // Any keystroke that is not a second Ctrl+W disarms a pending close,
        // so a confirmation cannot be answered by an unrelated key later.
        if event.is_key_down()
            && !(event.flags & MOD_CTRL != 0 && matches!(event.code, 0x57 | 0x77))
        {
            self.close_armed = false;
        }

        // The name entry owns the keyboard while it is open.
        if self.entry.is_some() && event.is_key_down() {
            return self.entry_key_event(event.code);
        }

        // A quick-open prompt is finished here rather than in the document,
        // because only the shell can see the tree it searches.
        if event.is_key_down()
            && event.code == KEY_ENTER
            && self.doc_ref().prompt == Some(PromptOp::Quick)
        {
            return self.finish_quick_open();
        }

        // Shell-level shortcuts, taken before the document sees them because
        // what they move belongs to the shell rather than to the text.
        if event.is_key_down() && event.flags & MOD_CTRL != 0 {
            // Ctrl+K is a chord prefix, as it is everywhere else: it commits to
            // nothing on its own and the next key decides. Anything other than
            // the keys it leads to clears it and is handled normally, so a
            // mistyped chord costs a keystroke rather than an edit.
            if self.chord_ctrl_k {
                self.chord_ctrl_k = false;
                if matches!(event.code, 0x54 | 0x74) {
                    super::theme::cycle();
                    return EventOutcome::Repaint;
                }
            } else if event.flags & MOD_SHIFT == 0 && matches!(event.code, 0x4B | 0x6B) {
                self.chord_ctrl_k = true;
                return EventOutcome::Repaint;
            }

            // Ctrl+B shows and hides the file tree. It is the shortcut people
            // reach for without looking, and the panel it moves lives on the
            // shell, while the document engine below only knows about text.
            if event.flags & MOD_SHIFT == 0 && matches!(event.code, 0x42 | 0x62) {
                self.sidebar_open = !self.sidebar_open;
                return EventOutcome::Repaint;
            }

            // Ctrl+P finds a file by part of its name. It is caught here
            // because resolving what was typed needs the file tree, which
            // belongs to the shell; the document engine only knows its own
            // text. Opening the prompt is all that happens now, and the Enter
            // that finishes it is intercepted below for the same reason.
            if event.flags & MOD_SHIFT == 0 && matches!(event.code, 0x50 | 0x70) {
                return super::path_prompt::start(self.doc(), PromptOp::Quick);
            }

            // Ctrl+N opens an empty document and Ctrl+W closes the one in
            // front. Both act on the tab strip, which is the shell's, not the
            // document's. Ctrl+W is deliberately not bound with Shift, so it
            // cannot be reached by a slip while selecting.
            if event.flags & MOD_SHIFT == 0 && matches!(event.code, 0x4E | 0x6E) {
                self.new_tab();
                return EventOutcome::Repaint;
            }
            if event.flags & MOD_SHIFT == 0 && matches!(event.code, 0x57 | 0x77) {
                let at = self.active;
                // A document with unsaved edits costs two presses. `close_tab`
                // removes it outright and there is no undo across a close, so
                // one slip next to Ctrl+S would be unrecoverable work. The
                // arming is cleared by any other key below.
                if self.docs[at].dirty && !self.close_armed {
                    self.close_armed = true;
                    self.docs[at].status = b"unsaved: Ctrl+W again to close, Ctrl+S to write";
                    return EventOutcome::Repaint;
                }
                self.close_armed = false;
                self.close_tab(at);
                return EventOutcome::Repaint;
            }
            self.close_armed = false;
        }

        if event.kind == InputKind::Wheel {
            return self.wheel_event(&event);
        }

        let pointer = matches!(
            event.kind,
            InputKind::ButtonDown | InputKind::PointerAbs | InputKind::ButtonUp
        );
        if pointer {
            if let Some(outcome) = self.pointer_event(&event) {
                return outcome;
            }
        }

        // Code pane and all remaining keyboard input: apply the pane rect,
        // then forward to the document engine.
        let ribbon = self.ribbon_shown();
        let (rx, ry, rw, rh) = pane_rect(self.last_w, self.last_h, self.sidebar_open, ribbon);
        let d = self.doc();
        d.pane_x = rx;
        d.pane_y = ry;
        d.pane_w = rw;
        d.pane_h = rh;
        on_event(d, event)
    }
}
