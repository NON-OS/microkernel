//! The address a sender needs: the spend key a note commits to, and the
//! viewing key its ciphertext is sealed to. Both halves are public.

mod layout;
mod parse;
mod recipient;
mod text;

use layout::{checksum, CHECK, PAYLOAD, PREFIX};

pub use recipient::Address;
