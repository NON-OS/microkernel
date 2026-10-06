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

use crate::browser::html::tokenizer::{Tag, TextMode};

use super::super::super::node::Ns;
use super::super::ops::scope::Scope;
use super::super::ops::state::Builder;

impl Builder {
    /// Start tags for raw text, select parts, ruby, foreign roots and
    /// ordinary elements.
    pub(in super::super) fn body_start_rest(&mut self, t: Tag) {
        match &*t.name {
            "button" => {
                if self.in_scope("button", Scope::Default) {
                    self.implied_end(None);
                    self.pop_until("button");
                }
                self.reconstruct();
                self.insert_html(t);
                self.frameset_ok = false;
            }
            "textarea" => self.textarea(t),
            "xmp" => {
                self.close_p_in_button_scope();
                self.reconstruct();
                self.frameset_ok = false;
                self.raw_element(t, TextMode::Rawtext);
            }
            "iframe" => {
                self.frameset_ok = false;
                self.raw_element(t, TextMode::Rawtext);
            }
            "noembed" => self.raw_element(t, TextMode::Rawtext),
            "select" => self.body_select(t),
            "option" | "optgroup" => self.body_option(t),
            "rb" | "rtc" | "rp" | "rt" => {
                if self.in_scope("ruby", Scope::Default) {
                    let keep = if matches!(&*t.name, "rp" | "rt") { Some("rtc") } else { None };
                    self.implied_end(keep);
                }
                self.insert_html(t);
            }
            "math" => self.foreign_root(t, Ns::MathMl),
            "svg" => self.foreign_root(t, Ns::Svg),
            "caption" | "col" | "colgroup" | "frame" | "head" | "tbody" | "td" | "tfoot" | "th"
            | "thead" | "tr" => {}
            _ => {
                self.reconstruct();
                self.insert_html(t);
            }
        }
    }
}
