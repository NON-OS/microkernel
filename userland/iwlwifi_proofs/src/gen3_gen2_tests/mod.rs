// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family (the AX201 CNVi on a Qu or QuZ MAC: 8086:02F0, 06F0,
//! 34F0, 3DF0, 4DF0, 43F0, A0F0; the discrete AX200, 8086:2723) on its own
//! boot path, host-proven only: the real linux-firmware images (API 77)
//! parse with the driver's own parser and differ from what it speaks only
//! where it now speaks theirs; selection and transports are cfg/22000.c's,
//! the subsystem rows of pcie/drv.c rebuilt and compared; the boot runs the
//! version 1 context info against a modeled 22000 part, whose rings,
//! packets, descriptors and command pointer are this family's; and a join
//! and data run on it end to end (`join_tests::gen2`). No 22000-family part
//! has booted it.

mod boot;
mod ctxt;
mod drv_rows;
mod firmware;
mod packets;
pub(crate) mod rig;
mod rings;
mod select;
mod start;
mod subsystems;
mod up;
