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
//! The frame the three custody screens share.

use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::state::State;

pub(super) fn frame<'a>(
    state: &'a State,
    title: &'a str,
    footer: &'a [(&'a str, Weight, bool)],
    status: &'a [&'a str],
) -> FrameSpec<'a> {
    FrameSpec {
        number: "02",
        title,
        back: true,
        backdrop: Some(Backdrop::Backup),
        failure: state.failure,
        footer,
        status,
        scroll: state.scroll,
    }
}
