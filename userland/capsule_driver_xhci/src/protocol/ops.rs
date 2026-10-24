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
pub const OP_HEALTHCHECK: u16 = 0x0001;
pub const OP_CONTROLLER_STATUS: u16 = 0x0002;
pub const OP_PORT_STATUS: u16 = 0x0003;
pub const OP_ENABLE_SLOT: u16 = 0x0004;
pub const OP_DISABLE_SLOT: u16 = 0x0005;
pub const OP_ADDRESS_DEVICE: u16 = 0x0006;
pub const OP_GET_DEVICE_DESCRIPTOR: u16 = 0x0007;
pub const OP_GET_CONFIG_DESCRIPTOR: u16 = 0x0008;
pub const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
pub const OP_CONTROL_TRANSFER: u16 = 0x000B;
pub const OP_INTERRUPT_IN: u16 = 0x000E;
pub const OP_CONFIGURE_BULK: u16 = 0x0010;
/// A second bulk pair on a slot, as OP_CONFIGURE_BULK: UAS's data pipes
/// beside its command and status pipes. Bulk ops name it as pair 1.
pub const OP_CONFIGURE_BULK2: u16 = 0x0023;
/// Streams on a pair's bulk endpoints (SuperSpeed UAS); bulk ops name one
/// in bits 7:4 of their pair byte.
pub const OP_CONFIGURE_STREAMS: u16 = 0x0024;
/// One report to an interrupt OUT endpoint, its ring from
/// OP_ALLOC_TRANSFER_RING: `slot, dci, len_le16, data`.
pub const OP_INTERRUPT_OUT: u16 = 0x0025;
/// `slot, port, interface`: the caller drives that interface of the
/// device; a second driver may then share the device (mux_claims.rs).
pub const OP_CLAIM_INTERFACE: u16 = 0x0026;
/// `port`: the shared slot there, `slot, 0, 0, 0, claimed u32`.
pub const OP_SLOT_OF_PORT: u16 = 0x0027;
pub const OP_BULK_OUT: u16 = 0x0011;
pub const OP_BULK_IN: u16 = 0x0012;
pub const OP_RESET_BULK: u16 = 0x0013;
/// The USB network drivers' read (`nonos_usbnet/src/xhci/wire.rs`): one look
/// at an armed bulk IN, answered at once. 0x20, 0x21 are the hub ops; the
/// isoch ops keep clear of them.
pub const OP_BULK_IN_POLL: u16 = 0x0014;
/// Address a device below a hub: `slot, root_port, speed, route_le32,
/// tt_hub_slot, tt_port`; replies as OP_ADDRESS_DEVICE.
pub const OP_ADDRESS_ROUTED: u16 = 0x0020;
/// Tell the controller a slot is a hub: `slot, ports, think_time, mtt`.
pub const OP_CONFIGURE_HUB: u16 = 0x0021;
pub const OP_CONFIGURE_ISOCH: u16 = 0x0018;
pub const OP_ISOCH_OUT: u16 = 0x0019;
pub const OP_ISOCH_IN: u16 = 0x001A;
/// The authorization gate's ops (slots/authorize.rs), the kernel's alone:
/// the devices held for the owner's answer, `count, (slot, ticket_le32)*`;
/// the answer, `slot, allow, ticket_le32`; and the policy, `0 allow, 1 ask,
/// 2 deny`.
pub const OP_USB_HELD: u16 = 0x0028;
pub const OP_USB_AUTHORIZE: u16 = 0x0029;
pub const OP_USB_POLICY: u16 = 0x002A;
