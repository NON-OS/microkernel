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

//! A model SD memory card for the SDHCI model to hold in a removable slot.

mod card;
mod cfg;
mod command;
mod dev;
mod flags;
mod ident;
mod op_cond;
mod regs;
mod rw;
mod sdsc;
mod state;
mod tran;

pub use card::SdCard;
pub use cfg::SdCfg;
pub use sdsc::sdsc;
