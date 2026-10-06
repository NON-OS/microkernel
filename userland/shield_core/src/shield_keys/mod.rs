//! The keys an account holds and the address it publishes. All descend from a single seed,
//! so a restored phrase restores every key and every note the wallet can find.

mod account;
#[cfg(test)]
#[path = "accounts_test.rs"]
mod accounts_test;
mod address;
pub(crate) mod base32;
pub(crate) mod base32_read;
pub(crate) mod derive;
mod domain;
pub(crate) mod material;
mod nullifier;
mod receive;
mod receiving_address;
#[cfg(test)]
#[path = "receiving_address_test.rs"]
mod receiving_address_test;
mod redact;
pub(crate) mod view;
pub(crate) mod view_key;
pub(crate) mod view_key_read;
#[cfg(test)]
#[path = "view_key_refusals_test.rs"]
mod view_key_refusals_test;
#[cfg(test)]
#[path = "view_key_test.rs"]
mod view_key_test;
pub(crate) mod view_key_text;
mod viewer;

pub use account::Account;
pub use address::Address;
pub(crate) use derive::store_key;
pub use nullifier::{note_nullifier_wire, nullifier_wire_with};
pub use receive::{ReceiveKey, ENCAPSULATION_KEY_BYTES};
pub use receiving_address::{
    parse_receiving_address, receiving_address, receiving_address_text, RECEIVING_ADDRESS_BYTES,
};
pub use view::ViewKey;
pub use view_key::ViewKind;
pub use viewer::Viewer;
