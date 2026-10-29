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
//! The core's start as the window shows it: the name asked for when the
//! core has no profile yet, handed to the start once given, and why it
//! stopped when it did (model/start.rs).

use alloc::format;
use alloc::string::String;

use super::app::SimplexUi;
use crate::model::focus::Focus;
use crate::model::start::Phase;
use crate::model::start_name::NAME_MAX;

impl SimplexUi {
    /// True when the window must repaint.
    pub fn follow_start(&mut self) -> bool {
        let model = &mut self.model;
        let Some(start) = self.link.start() else { return false };
        let say = match (start.phase.clone(), model.focus) {
            (Phase::Ask, Focus::Named) if start.named(&model.name) => "starting SimpleX".into(),
            (Phase::Ask, Focus::Named) => {
                model.focus = Focus::Name;
                format!("a name of at most {NAME_MAX} bytes, with no control characters")
            }
            (Phase::Ask, f) if f != Focus::Name => {
                model.focus = Focus::Name;
                "SimpleX has no profile on this computer yet".into()
            }
            (Phase::Up, Focus::Name | Focus::Named) => {
                model.focus = Focus::List;
                String::new()
            }
            (Phase::Stopped(why), _) if model.notice != why => {
                if matches!(model.focus, Focus::Name | Focus::Named) {
                    model.focus = Focus::List;
                }
                String::from(why)
            }
            _ => return false,
        };
        model.notice = say;
        true
    }
}
