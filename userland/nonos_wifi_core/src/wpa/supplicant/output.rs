// NONOS Operating System (AGPL-3.0-or-later)

//! What one step of the handshake produced.

use alloc::vec::Vec;

/// What `step` produced: a frame to transmit, and whether the group key
/// changed. A frame is returned for messages 2 and 4 and for group message 2;
/// `None` means the input was not a message the supplicant acts on, or it was
/// dropped (check `Supplicant::state` for `Failed`).
pub struct StepOutput {
    pub reply: Option<Vec<u8>>,
    /// A group key handshake delivered a new group key: the caller installs
    /// `Supplicant::gtk` at `Supplicant::gtk_id`.
    pub new_group_key: bool,
    /// A rekey finished: the caller sends `reply` (message 4) under the
    /// pairwise key in use, then installs `Supplicant::tk` in its place.
    pub new_pairwise_key: bool,
}

impl StepOutput {
    pub(super) fn none() -> Self {
        Self { reply: None, new_group_key: false, new_pairwise_key: false }
    }

    pub(super) fn reply(frame: Option<Vec<u8>>) -> Self {
        Self { reply: frame, new_group_key: false, new_pairwise_key: false }
    }
}
