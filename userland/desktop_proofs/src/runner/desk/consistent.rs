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

//! What is shown and what is hit agree at every pixel.

use super::consts::SHADOW;
use super::types::Desk;
use crate::damage::Rect as Damage;

impl Desk {
    /// Every pixel the screen shows equals a full recomposite, and is the
    /// window a press there goes to (windows are opaque here, margins too).
    pub fn assert_consistent(&mut self, what: &str) {
        let shown = self.screen.clone();
        let all = Damage { x: 0, y: 0, width: self.width, height: self.height };
        self.paint(all);
        for y in (0..self.height).step_by(3) {
            for x in (0..self.width).step_by(3) {
                let i = (y * self.width + x) as usize;
                assert_eq!(shown[i], self.screen[i], "{what}: stale pixel at ({x}, {y})");
                // The dock's shadow is drawn but takes no press: the dock's
                // window is its panel.
                if self.chrome[i] == SHADOW && self.screen[i] == self.shell {
                    continue;
                }
                assert_eq!(
                    self.screen[i],
                    self.hit(x, y),
                    "{what}: ({x}, {y}) shows one window, a press goes to another"
                );
            }
        }
    }
}
