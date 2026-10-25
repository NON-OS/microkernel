//! The address as a value: the spend key and the viewing key.

/// A recipient: what a sender needs to build a note only that account can spend
/// and only that account can find.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Address {
    pub spend_pk: [u64; 4],
    pub view_pk: [u8; 32],
}
