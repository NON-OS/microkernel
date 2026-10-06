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

//! The host commands that take the running firmware from a scanning
//! interface to a station joined to one access point, and back. The bundled
//! SO images (so-a0-gf-a0-86, so-a0-hr-b0-84) speak the MLD API
//! (`IWL_UCODE_TLV_CAPA_MLD_API_SUPPORT`), so the join is built as Linux
//! v6.12 builds it on that API (mvm/mld-mac80211.c, mld-mac.c, link.c,
//! mld-sta.c, mld-key.c, phy-ctxt.c, time-event.c, tx.c, pcie/tx-gen2.c):
//!
//! - a PHY context on the access point's channel (`phy`), with the receive
//!   chains in RLC_CONFIG_CMD;
//! - the link pointed at that PHY and made active with the BSS's ACK rates
//!   (`link`, `rates`). On the MLD API there is no BINDING_CONTEXT_CMD: the
//!   link's `phy_id` is the binding, and the firmware lists no binding
//!   command at all;
//! - session protection to keep the radio on channel while joining (`session`);
//! - the access point as a peer station (`sta`) with its management and data
//!   transmit queues (`queue`);
//! - after association, the MAC context marked associated (`mac`);
//! - the pairwise and group keys (`key`);
//! - the scan abort a join sends when the network was found mid-sweep
//!   (`abort`).
//!
//! Each builder returns the command's little-endian byte image and names the
//! `struct` and API version it encodes; `ids` holds the group, opcode and
//! version of each, and refuses a firmware that reports another version.

pub mod abort;
pub mod ids;
pub mod key;
pub mod link;
pub mod mac;
pub mod phy;
pub mod queue;
pub mod rates;
pub mod session;
pub mod sta;
