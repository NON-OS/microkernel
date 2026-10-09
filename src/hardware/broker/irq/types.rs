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

//! Wire and kernel-side types for `MkIrqBind`. The `flags` field on
//! the request selects the delivery path: legacy INTx (flags == 0),
//! MSI-X (`BIND_MSIX`) or MSI (`BIND_MSI`), never two at once.

// The error types live in errors.rs; every arch backend names them here.
pub(super) use super::errors::{IrqBindError, IrqError, IrqPollResult};

// Public flag bits for `IrqBindRequest::flags`. The kernel rejects
// any unset bit so capsules cannot quietly opt into a future flag
// they were not designed against.
pub const BIND_MSIX: u32 = 1 << 0;
pub const BIND_MSI: u32 = 1 << 1;
pub const FLAGS_KNOWN: u32 = BIND_MSIX | BIND_MSI;

#[derive(Debug, Clone, Copy)]
pub struct IrqBindRequest {
    pub device_id: u64,
    pub claim_epoch: u64,
    // INTx mode: GSI from `mk_device_list`.
    // MSI-X and MSI mode: must be 0; the kernel programs the MSI-X
    // table from entry 0, or the MSI capability, for the device.
    pub irq_source: u32,
    pub flags: u32,
    // INTx mode: must be 0.
    // MSI-X mode: 1..=BROKER_VEC_COUNT, capped further by the
    // device's MSI-X table size.
    pub vector_count: u32,
}

// `IrqBindResult` is the base of an N-vector range. For INTx N is
// always 1. For MSI-X N == request.vector_count and the capsule
// derives the per-vector grant IDs as `grant_id + i` and vectors
// as `vector + i` for i in 0..vector_count.
#[derive(Debug, Clone, Copy)]
pub struct IrqBindResult {
    pub grant_id: u64,
    pub vector: u8,
}
