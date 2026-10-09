//! Where a swap trades: Uniswap V2 on mainnet, its router, WETH, and the two
//! pairs every route uses, each read from the chain. A pair's `token0` is
//! checked on every quote, and an order that differs from this file stops the swap.

use crate::evm::const_hex::hex20;

/// One pair, and the token its reserves list first.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pair {
    pub address: [u8; 20],
    pub token0: [u8; 20],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Venue {
    pub router: [u8; 20],
    pub weth: [u8; 20],
    pub nox_weth: Pair,
    pub usdc_weth: Pair,
}

/// Uniswap V2 on mainnet. The NOX token names this router too.
pub const MAINNET: Venue = Venue {
    router: hex20("7a250d5630b4cf539739df2c5dacb4c659f2488d"),
    weth: hex20("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
    nox_weth: Pair {
        address: hex20("07ce5889d2eb681af3bd61db24ab2602c502bd1b"),
        token0: hex20("0a26c80be4e060e688d7c23addb92cbb5d2c9eca"),
    },
    usdc_weth: Pair {
        address: hex20("b4e16d0168e52d35cacd2c6185b44281ec28c9dc"),
        token0: hex20("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
    },
};
