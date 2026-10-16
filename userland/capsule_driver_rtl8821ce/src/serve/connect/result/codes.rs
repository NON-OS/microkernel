// NONOS Operating System (AGPL-3.0-or-later)

//! The status codes a join answers with.

/// The radio is down or the request was malformed.
pub const CODE_BAD_REQUEST: i32 = -1;

/// The network was not heard on any channel.
pub const CODE_NOT_FOUND: i32 = -2;

/// The access point refused authentication or association.
pub const CODE_REFUSED: i32 = -5;

/// The handshake did not finish (a WPA2 passphrase mismatch ends here).
pub const CODE_TIMED_OUT: i32 = -6;

/// The network was saved as WPA3 and now offers only WPA2: not joined.
pub const CODE_DOWNGRADE: i32 = -7;

/// The network's security is not one this station runs (open, TKIP,
/// Enterprise, 802.11n-only).
pub const CODE_UNSUPPORTED: i32 = -8;

/// The passphrase is not 8 to 63 characters or 64 hex digits.
pub const CODE_BAD_PASSPHRASE: i32 = -9;

/// WPA3: the access point's SAE confirm did not verify (a wrong password).
pub const CODE_WRONG_PASSWORD: i32 = -10;

/// Message 3 did not carry the beacon's security elements: possibly an
/// attempted downgrade; the join was abandoned.
pub const CODE_IE_MISMATCH: i32 = -11;

/// No randomness for the station's nonces and SAE secrets.
pub const CODE_NO_ENTROPY: i32 = -12;
