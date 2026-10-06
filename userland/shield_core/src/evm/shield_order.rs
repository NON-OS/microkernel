//! A deposit from the public account: what is asked for, and what the review answers.

use super::tx::Eip1559;

/// What to deposit: into `pool`, of `token` or of ether when none, and the two calls to make.
pub struct ShieldOrder<'a> {
    pub pool: [u8; 20],
    pub token: Option<[u8; 20]>,
    pub amount: u128,
    pub approve_data: &'a [u8],
    pub deposit_data: &'a [u8],
    /// What the screen says when the pool would refuse the call.
    pub refused_as: &'static str,
}

/// The transaction to sign, and whether it is the approval that comes first.
pub struct Shielding {
    pub tx: Eip1559,
    pub approval: bool,
    pub max_network_fee: u128,
}

pub enum ShieldChecked {
    Ready(Box<Shielding>),
    Refused(&'static str),
}
