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

//! Each interrupt endpoint's and each root port's places in the layout.

use super::layout::*;
use crate::facts::qh::QH_LEN;
use crate::facts::td::TD_LEN;

/// Interrupt endpoint `i`'s queue, TD and report buffer.
pub const fn intr_qh(i: usize) -> usize {
    INTR_QH + QH_LEN * i
}
pub const fn intr_td(i: usize) -> usize {
    INTR_TD + TD_LEN * i
}
pub const fn intr_buf(i: usize) -> usize {
    INTR_BUF + INTR_BUF_LEN * i
}

/// Polled place `place`: its queue, chain and buffer.
pub const fn poll_qh(place: usize) -> usize {
    BULK_QH + QH_LEN * (1 + place)
}
pub const fn poll_td(place: usize) -> usize {
    POLL_TD + TD_LEN * BULK_TDS * place
}
pub const fn poll_buf(place: usize) -> usize {
    POLL_BUF + BULK_MAX * place
}
