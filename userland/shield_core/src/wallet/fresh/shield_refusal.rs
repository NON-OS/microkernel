//! Why SHIELD_FROM_FRESH refuses (v1.4.56), in a sentence each.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShieldRefusal {
    UnknownAddress,
    /// Less than the smallest standard deposit is there.
    BelowStandard,
    NotPool,
    /// An approval of anything but the deposits' sum, to the pool.
    ApproveNotExact,
    /// Ether sent with an approval or a NOX deposit, an ETH deposit with
    /// none, or values that do not sum to the deposits'.
    Value,
    FeesPastCaps,
}

impl ShieldRefusal {
    pub fn why(self) -> &'static str {
        match self {
            ShieldRefusal::UnknownAddress => "That address was not handed out by this shield.",
            ShieldRefusal::BelowStandard => {
                "Less than the smallest standard deposit is there; it stays at that address."
            }
            ShieldRefusal::NotPool => "A fresh address shields only into the config's pool.",
            ShieldRefusal::ApproveNotExact => "The approval must be exactly the deposits' sum.",
            ShieldRefusal::Value => "Ether goes only as an ETH deposit's own value.",
            ShieldRefusal::FeesPastCaps => "The gas or the fees are past the caps.",
        }
    }
}
