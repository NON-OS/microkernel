//! The words around a swap quote: the rate per whole coin, and the pools it
//! passes through, as a screen shows them.

use crate::evm::swap::Swap;
use crate::net::asset::Coin;

/// "1 ETH = 823,180.98 NOX": what one whole coin in buys at the quote, to
/// two decimals, with thousands grouped by commas.
pub(super) fn rate(swap: &Swap, expected: u128) -> String {
    let one = 10u128.checked_pow(swap.from.decimals()).unwrap_or(1);
    let per = crate::evm::swap::math::mul_div(expected, one, swap.amount).unwrap_or(0);
    let hundredths = 10u128.checked_pow(swap.to.decimals().saturating_sub(2)).unwrap_or(1);
    let cents = per.checked_div(hundredths).unwrap_or(0);
    let (whole, part) = (cents.checked_div(100).unwrap_or(0), cents.checked_rem(100).unwrap_or(0));
    format!("1 {} = {}.{part:02} {}", swap.from.symbol(), grouped(whole), swap.to.symbol())
}

pub(super) fn grouped(n: u128) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && digits.len().saturating_sub(i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// "NOX/WETH · 0x07CE…Bd1B" for each pair on the route.
pub(super) fn pools(swap: &Swap, pairs: &[[u8; 20]]) -> Vec<String> {
    let names: Vec<&str> = match (swap.from, swap.to) {
        (Coin::Nox, Coin::Usdc) => vec!["NOX/WETH", "USDC/WETH"],
        (Coin::Usdc, Coin::Nox) => vec!["USDC/WETH", "NOX/WETH"],
        (Coin::Nox, _) | (_, Coin::Nox) => vec!["NOX/WETH"],
        _ => vec!["USDC/WETH"],
    };
    names
        .iter()
        .zip(pairs)
        .map(|(name, pair)| {
            let full = crate::evm::checksummed(pair);
            let (head, tail) = (full.get(..6).unwrap_or(""), full.get(38..).unwrap_or(""));
            format!("{name} · {head}…{tail}")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{grouped, rate};
    use crate::evm::swap::Swap;
    use crate::net::asset::Coin;

    #[test]
    fn the_rate_reads_per_whole_coin_with_grouped_thousands() {
        let swap = Swap {
            from: Coin::Eth,
            to: Coin::Nox,
            amount: 1_000_000_000_000_000,
            slippage_bps: 50,
        };
        assert_eq!(rate(&swap, 823_180_985_747_139_496_380), "1 ETH = 823,180.98 NOX");
        let usdc = Swap { to: Coin::Usdc, ..swap };
        assert_eq!(rate(&usdc, 2_678_186), "1 ETH = 2,678.18 USDC");
        assert_eq!(grouped(0), "0");
        assert_eq!(grouped(1_000), "1,000");
    }
}
