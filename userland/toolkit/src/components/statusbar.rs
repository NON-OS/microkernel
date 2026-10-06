#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusFlags {
    pub network_up: bool,
    /// None when there is no reading; never a stand-in 0.
    pub battery_pct: Option<u8>,
    pub alerts: u8,
}

impl StatusFlags {
    pub fn battery_clamped(mut self) -> Self {
        self.battery_pct = self.battery_pct.map(|p| p.min(100));
        self
    }
}
