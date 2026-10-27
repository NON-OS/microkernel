// NONOS Operating System (AGPL-3.0-or-later)
//! Ways to make the modelled chip misbehave, one at a time, so a proof can
//! show the driver stops where it must and says so.

#[derive(Default)]
pub struct Faults {
    /// (word, bits, level): those bits read as `level`, whatever was written.
    pub stuck: Vec<(u32, u32, bool)>,
    /// The firmware's status after a download, in place of ready (7).
    pub fw_status: Option<u8>,
    /// The firmware never answers the register mailbox.
    pub mute: bool,
    /// The DMA engine never takes the firmware command queue (CH12).
    pub h2c_stall: bool,
    /// The DMA engine never takes the data and management queues.
    pub tx_stall: bool,
    /// Frames go out but no release report comes back.
    pub no_release: bool,
    /// The efuse cells never come ready.
    pub efuse_dead: bool,
    /// Every word reads all ones: the card has left the bus.
    pub gone: bool,
    /// A protected frame no key slot opens comes up as it was heard.
    pub pass_unkeyed: bool,
    /// The engine stops at a security CAM command and takes nothing more.
    pub sec_stall: bool,
}

impl Faults {
    /// `v` as read at word `a`, with any stuck bits forced.
    pub fn apply(&self, a: u32, mut v: u32) -> u32 {
        if self.gone {
            return u32::MAX;
        }
        for &(w, bits, level) in &self.stuck {
            if w == a {
                v = if level { v | bits } else { v & !bits };
            }
        }
        v
    }
}
