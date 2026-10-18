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

//! Hardware reset + IRQ quiesce + link bring-up, in the order the 8254x
//! needs on silicon: steps 1 to 3 are `reset_part`; then every cause is
//! masked again, the causes latched meanwhile cleared, and the link set up.

use crate::regs::Regs;

use super::{link_up, reset_part};

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    reset_part::run(regs)?;
    link_up::run(regs);
    Ok(())
}
