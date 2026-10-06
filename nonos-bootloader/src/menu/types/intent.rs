// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::action::MenuAction;

/*
 * What the person asked the verified kernel to do once it runs. It is kept
 * apart from SecurityMode on purpose: installing changes what the kernel
 * starts first, never how the kernel is checked, so an install boot goes
 * through exactly the signature, attestation and rollback checks of the
 * security mode it was resolved to.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BootIntent {
    #[default]
    Run,
    Install,
}

impl BootIntent {
    pub const fn of(action: MenuAction) -> Self {
        match action {
            MenuAction::Install => Self::Install,
            _ => Self::Run,
        }
    }
}
