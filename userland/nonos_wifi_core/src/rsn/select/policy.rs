// NONOS Operating System (AGPL-3.0-or-later)

//! What the person allowed for a join.

/// What the person allowed for this join.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct JoinPolicy {
    /// WPA2-Personal (PSK, PSK-SHA256) may be used. False for a network saved
    /// as WPA3: then only SAE is acceptable.
    pub allow_psk: bool,
    /// WPA3-Personal (SAE) may be used.
    pub allow_sae: bool,
}

impl JoinPolicy {
    /// The default: SAE when offered, PSK otherwise.
    pub const ANY: JoinPolicy = JoinPolicy { allow_psk: true, allow_sae: true };
    /// A network saved as WPA3: SAE only, never downgraded.
    pub const WPA3_ONLY: JoinPolicy = JoinPolicy { allow_psk: false, allow_sae: true };
    /// WPA2-PSK only, for a caller that cannot supply SAE's randomness.
    pub const PSK_ONLY: JoinPolicy = JoinPolicy { allow_psk: true, allow_sae: false };
}
