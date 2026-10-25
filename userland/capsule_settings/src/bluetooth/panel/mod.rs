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

//! The Bluetooth panel's own logic, apart from the calls it makes, so the
//! proofs run all of it: what the service last said, what each key asks of
//! it, and the words the page reads.

mod act;
pub mod addr;
mod answer;
mod did;
pub mod digits;
pub mod key;
mod known;
mod left;
pub mod notice;
mod reach;
mod request;
mod said;
mod take;

pub use act::act;
pub use addr::{addr_text, made};
pub use digits::DIGITS;
pub use key::bt_key;
pub use known::Panel;
pub use left::{level_of, poll_ms, window};
pub use notice::{refused, unreached};
pub use reach::Reach;
pub use request::{Request, Step};
pub use said::{controller, scanning, status};
