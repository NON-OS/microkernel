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

//! From a reset host to a card in transfer state, in the order Linux's
//! mmc_attach_mmc and mmc_init_card take: power at the host's highest
//! voltage, query the OCR, settle on the lowest shared voltage, CMD1 until
//! ready, CID, RCA, CSD, select, EXT_CSD, user area, speed and bus width.

mod bring_up;
mod capacity;
mod check_ready;
mod op_cond;
mod power;
mod power_on;
mod query_ocr;
mod read_cid;
mod read_ext;
mod select_card;
mod step;
mod user_area;
mod voltage;

pub use bring_up::bring_up;
