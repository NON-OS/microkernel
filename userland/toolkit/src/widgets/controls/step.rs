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

//! What one input event asks of the app.

/// What one input event asks of the app: nothing, a repaint, or an action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Step<A> {
    /// Nothing anyone can see changed.
    Idle,
    /// A control's hover or press changed, so the app repaints.
    Repaint,
    /// The control for `A` was clicked. The app acts on it and repaints.
    Act(A),
}
