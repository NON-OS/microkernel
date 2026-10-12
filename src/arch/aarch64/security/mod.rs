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

pub mod bti;
pub mod init_all;
pub mod mte;
pub mod pac;
pub mod pan;
pub mod report;
pub mod spectre;

pub use bti::{init_bti, kernel_guarded};
pub use init_all::init_all;
pub use mte::{init_mte, MteMode};
pub use pac::{active as pac_active, enter_process};
pub use pan::init_pan;
pub use report::report;
pub use spectre::init_spectre_mitigations;
