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

//! The function: what enumeration found (caps.rs `ResetCaps`), how it behaves
//! in this seed, and its state: Command, Device Control, its power state,
//! when it answers again, when its pending transactions finish, the state
//! only a reset clears (`secret`, the service whose it is) and the runs its
//! queue writes into.

use super::method_rule::Offers;

/// `af_tp`: an Advanced Features capability that reports Transactions
/// Pending; `untranslated`: VMMDev, which reads guest memory past any IOMMU.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Caps {
    pub offers: Offers,
    pub pcie: bool,
    pub af_tp: bool,
    pub pm: bool,
    pub ident: u32,
    pub display: bool,
    pub binds: bool,
    pub untranslated: bool,
    pub mem_bar: bool,
}

/// What the function truly is, apart from what the kernel read of it:
/// `flr`, `af_flr` and `soft_d3` (No_Soft_Reset clear) as its registers
/// hold them, `past_bm` whether it reaches memory past Bus Master off.
/// `ignores_flr`, `ignores_d3`: such a reset does nothing to it anyway;
/// `stuck_bm`: Bus Master, once set, will not clear; `retry`: it answers
/// the retry status 0x0001, not all ones, until it is back (recalled,
/// unverified); `tp_never`: transactions it has pending never finish.
#[derive(Clone, Debug, Default)]
pub(super) struct Dev {
    pub caps: Caps,
    pub flr: bool,
    pub af_flr: bool,
    pub soft_d3: bool,
    pub past_bm: bool,
    pub ignores_flr: bool,
    pub ignores_d3: bool,
    pub stuck_bm: bool,
    pub intx_sticks: bool,
    pub retry: bool,
    pub tp_never: bool,
    pub cmd: u16,
    pub devctl: u16,
    pub pm_state: u16,
    pub ready_at: u64,
    /// The end of the window a reset write opened (I3).
    pub quiet_until: u64,
    pub tp_until: u64,
    pub secret: Option<u8>,
    pub queue: Vec<usize>,
    /// A holder's driver runs it, so stopping it leaves transactions pending.
    pub serving: bool,
}
