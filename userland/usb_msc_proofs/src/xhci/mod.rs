// NONOS Operating System (AGPL-3.0-or-later)
//! driver.xhci0 as the driver's `xhci` module gives it, scripted: one
//! Bulk-Only device on a root port, with the calls and the errnos
//! `capsule_driver_usb_msc/src/xhci` gives the disk and scan code, each
//! spending the controller's time on the test's clock. The driver's own
//! call and slot requests run over a scripted IPC call (`ipc`).

mod bulk;
#[path = "../../../capsule_driver_usb_msc/src/xhci/call.rs"]
mod call;
mod control;
mod device;
mod host_rig;
mod ipc;
mod pairs;
#[path = "../../../capsule_driver_usb_msc/src/xhci/port_list.rs"]
pub mod port_list;
mod ports;
#[path = "../../../capsule_driver_usb_msc/src/xhci/reply.rs"]
pub mod reply;
#[path = "../../../capsule_driver_usb_msc/src/xhci/slot.rs"]
mod slot;
mod unready;
mod window;
#[path = "../../../capsule_driver_usb_msc/src/xhci/wire.rs"]
pub mod wire;

pub use bulk::{bulk_in, bulk_out, configure_bulk, reset_bulk};
pub use control::{config_descriptor, control_in, control_no_data};
pub use device::{cbw_takes, device, script, setup, Answer, Device};
pub use host_rig::{host, hosts, Hosts};
pub(crate) use ipc::mk_ipc_call_timeout;
pub use pairs::{bulk_in_on, bulk_out_on, configure_bulk2, configure_streams, port_speed};
pub use pairs::{reset_bulk_on, DATA_PAIR};
pub use port_list::Port;
pub use ports::connected_ports;
pub use slot::{address_device, disable_slot, enable_slot};
pub use window::{left_ms, Window, XHCI_GIVES_UP_MS};
pub use wire::{BULK_MAX, E_BUSY, E_PIPE, PORT_CLAIMED, PORT_FREE};
