// NONOS Operating System (AGPL-3.0-or-later)

//! A connection attempt's result as the client reads it.

/// A connection attempt's result: the status code, and the handshake progress
/// (frames sent and received, and the state reached) for the panel to show,
/// then the AKM that ran (0 none, 2 WPA2-PSK, 6 PSK-SHA256, 8 WPA3-SAE) and
/// the access point's own status or reason code, when it gave one.
#[derive(Default)]
pub struct ConnectResult {
    pub code: i32,
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub probe: u32,
    pub deauth: u32,
    pub to_us: u32,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

impl ConnectResult {
    /// A result with only a status code.
    pub fn code(code: i32) -> Self {
        Self { code, ..Default::default() }
    }
}
