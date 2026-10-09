//! The pool arithmetic, in 256 bits where a product needs it. `in * 997 * out`
//! reaches 250 bits and is carried in crypto-bigint's U256. Only the final
//! division rounds, down, as the pair does, and every function refuses instead of wrapping.

use k256::elliptic_curve::bigint::{CheckedMul, NonZero, U256};

fn wide(v: u128) -> U256 {
    U256::from_u128(v)
}

fn narrow(v: U256) -> Option<u128> {
    let words = v.to_words();
    let high = words.get(2..).unwrap_or_default();
    if high.iter().any(|w| *w != 0) {
        return None;
    }
    let (w0, w1) = (*words.first()?, *words.get(1)?);
    Some(u128::from(w0) | (u128::from(w1) << 64))
}

/// `a * b / c`, rounded down, or none on a zero divisor or a result past 128 bits.
pub fn mul_div(a: u128, b: u128, c: u128) -> Option<u128> {
    let product = Option::<U256>::from(wide(a).checked_mul(&wide(b)))?;
    let divisor = Option::<NonZero<U256>>::from(NonZero::new(wide(c)))?;
    narrow(product.wrapping_div(&divisor))
}

/// A Uniswap V2 payout after the 0.3% fee: `in·997·r_out / (r_in·1000 + in·997)`.
pub fn amount_out(input: u128, r_in: u128, r_out: u128) -> Option<u128> {
    if input == 0 || r_in == 0 || r_out == 0 {
        return None;
    }
    let with_fee = input.checked_mul(997)?;
    let denominator = r_in.checked_mul(1000)?.checked_add(with_fee)?;
    mul_div(with_fee, r_out, denominator)
}

/// The price impact in basis points against the pre-trade price `input·r_out/r_in`.
pub fn impact_bps(input: u128, out: u128, r_in: u128, r_out: u128) -> Option<u128> {
    let at_price = mul_div(input, r_out, r_in)?;
    if at_price == 0 || out >= at_price {
        return Some(0);
    }
    mul_div(at_price.checked_sub(out)?, 10_000, at_price)
}

#[cfg(test)]
#[path = "math_test.rs"]
mod math_test;
