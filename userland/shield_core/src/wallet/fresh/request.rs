//! SIGN_FROM_FRESH's request and its refusals (plan/27, v1.4.51): a
//! transaction from a fresh address this wallet handed out, for the Escrow's
//! refund or dispute only.

/// What the wallet asks signed. `to` and `value` are asked only to be
/// checked: the Escrow is the config's, and nothing is ever sent with it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FreshSign {
    pub address: [u8; 20],
    pub chain_id: u64,
    pub nonce: u64,
    pub gas: u64,
    pub max_fee: u128,
    pub max_priority: u128,
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FreshRefusal {
    /// Not an address OP_FRESH_ADDRESS handed out.
    UnknownAddress,
    /// No Escrow on that chain in the config.
    NotConfigChain,
    OtherTo,
    Value,
    /// Neither refund nor dispute.
    OtherSelector,
    /// A refund paying an address other than the one signing.
    OtherRefundTo,
    /// Gas past the kind's limit, or a fee past the meter's caps.
    FeesPastCaps,
}

impl FreshRefusal {
    pub fn why(self) -> &'static str {
        match self {
            FreshRefusal::UnknownAddress => "That address was not handed out by this shield.",
            FreshRefusal::NotConfigChain => "That chain has no Escrow in the config.",
            FreshRefusal::OtherTo => "A fresh address signs only for the config's Escrow.",
            FreshRefusal::Value => "A fresh address sends no ether with a refund or dispute.",
            FreshRefusal::OtherSelector => "A fresh address signs only a refund or a dispute.",
            FreshRefusal::OtherRefundTo => "That refund pays another address than the one signing.",
            FreshRefusal::FeesPastCaps => "The gas or the fees are past the caps.",
        }
    }
}
