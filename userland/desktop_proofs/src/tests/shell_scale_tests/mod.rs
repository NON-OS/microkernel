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

//! The desktop's type and geometry scale as first-boot setup's do on the same
//! canvas: 1.25 from a short side of 1000, 1.5 from 1440, 2 from 2160, read
//! off the canvas the compositor hands out (half the panel from 2560 by 1440).

mod canvas;
mod chrome_sizes;
mod latch;
mod sizes;
