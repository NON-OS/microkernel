// NONOS Operating System (AGPL-3.0-or-later)
//! The pure part of the search: which ports to try, on every host.

#[path = "../../../nonos_usbnet/src/scan/book.rs"]
pub mod book;
#[path = "../../../nonos_usbnet/src/scan/port.rs"]
pub mod port;
#[path = "../../../nonos_usbnet/src/scan/search.rs"]
pub mod search;

pub use book::{Book, TRIES};
pub use port::Port;
pub use search::{search, Tried};
