//! Pool assets, in note units of `scale` base units. A deposit rereads `scale`, refusing a change.

/// The coins a screen names. USDC lives in the public account only, since no pool registers it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Coin {
    Eth,
    Nox,
    Usdc,
}

impl Coin {
    pub fn symbol(self) -> &'static str {
        match self {
            Coin::Eth => "ETH",
            Coin::Nox => "NOX",
            Coin::Usdc => "USDC",
        }
    }

    pub fn decimals(self) -> u32 {
        match self {
            Coin::Eth | Coin::Nox => 18,
            Coin::Usdc => 6,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Asset {
    pub coin: Coin,
    pub id: u64,
    pub scale: u128,
    /// The largest relayer fee a spend may pay, in note units.
    pub max_relay_fee: u64,
    /// The token contract, or none for the chain's coin, deposited as value.
    pub token: Option<&'static str>,
    /// The fee every spend pays, the same for all, so it never tells two wallets apart. Note units.
    pub relay_fee: u64,
    /// The smallest and largest amount a deposit, a send or a withdrawal may move. Note units.
    pub least: u64,
    pub most: u64,
}

/// ETH on the launch pool: counted in wei, relay fee capped at 0.001 ETH.
pub const ETH: Asset = Asset {
    coin: Coin::Eth,
    id: 0,
    scale: 1,
    max_relay_fee: 1_000_000_000_000_000,
    token: None,
    relay_fee: 100_000_000_000_000,
    least: 1_000_000_000_000_000,
    most: u64::MAX,
};

/// NOX on the launch pool: note units of 10^9 base units, relay fee capped at 10 NOX.
pub const NOX: Asset = Asset {
    coin: Coin::Nox,
    id: 1,
    scale: 1_000_000_000,
    max_relay_fee: 10_000_000_000,
    token: Some("0x3E5249A65CA513D5e11260222e0D26f46b465d36"),
    relay_fee: 1_000_000_000,
    least: 1_000_000,
    most: u64::MAX,
};

pub const FORMAT5_NOX: Asset = Asset { scale: 1, max_relay_fee: 0, ..NOX };
