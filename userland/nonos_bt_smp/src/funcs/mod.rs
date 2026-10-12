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

//! The four LE Secure Connections functions (Vol 3, Part H, 2.2.6 to
//! 2.2.9), and the 56-bit address form f5 and f6 take.

mod addr;
mod f4;
mod f5;
mod f6;
mod g2;

pub use addr::addr56;
pub use f4::f4;
pub use f5::{f5, SALT};
pub use f6::f6;
pub use g2::{g2, passkey_display};
