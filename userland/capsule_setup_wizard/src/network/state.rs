//! What the network step holds between keys.

use nonos_wifi_client::{wipe, Driver, DriverStage, SavedError, ScanNetwork, PASS_MAX};

/// The most scanned networks listed under "No network".
pub const NETS_MAX: usize = 6;

pub struct NetState {
    pub driver: Option<Driver>,
    pub stage: Option<DriverStage>,
    pub nets: [ScanNetwork; NETS_MAX],
    pub count: usize,
    /// Row: 0 is "No network", 1 and on are `nets`.
    pub sel: u8,
    /// The passphrase editor is open for the selected network.
    pub typing: bool,
    pub pass: [u8; PASS_MAX],
    pub pass_len: usize,
    /// The network joined in this step, held until review commits.
    pub joined: Option<ScanNetwork>,
    /// A join is running; the screen says so while the call blocks.
    pub joining: bool,
    /// The last join's status code, for the line that says what happened.
    pub result: Option<i32>,
    /// Remember the joined network. Offered only when the mode keeps state.
    pub remember: bool,
    /// Why remembering cannot be turned on, when it was tried and cannot.
    pub cannot_keep: Option<SavedError>,
    /// When the list was last refreshed, in milliseconds since boot.
    pub polled_ms: i64,
}

impl NetState {
    pub const fn new() -> Self {
        Self {
            driver: None,
            stage: None,
            nets: [ScanNetwork::EMPTY; NETS_MAX],
            count: 0,
            sel: 0,
            typing: false,
            pass: [0; PASS_MAX],
            pass_len: 0,
            joined: None,
            joining: false,
            result: None,
            remember: false,
            cannot_keep: None,
            polled_ms: 0,
        }
    }

    /// The network on the selected row, if the row is one.
    pub fn selected(&self) -> Option<ScanNetwork> {
        let i = (self.sel as usize).checked_sub(1)?;
        (i < self.count).then(|| self.nets[i])
    }

    /// Forget what was typed, from memory as well as from the screen.
    pub fn wipe_pass(&mut self) {
        wipe(&mut self.pass);
        self.pass_len = 0;
        self.typing = false;
    }
}
