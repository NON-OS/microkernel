//! Amounts as a person reads them: whole units, a point, and no trailing
//! zeros. Both coins have eighteen decimals, and a fee per gas reads in gwei.

/// `value` counted in units of 10^-`decimals`, as text.
pub(crate) fn format(value: u128, decimals: u32) -> String {
    let scale = 10u128.checked_pow(decimals).unwrap_or(1);
    let whole = value.checked_div(scale).unwrap_or(0);
    let fraction = value.checked_rem(scale).unwrap_or(0);
    if fraction == 0 {
        return whole.to_string();
    }
    let width = usize::try_from(decimals).unwrap_or(0);
    let text = format!("{whole}.{fraction:0width$}");
    text.trim_end_matches('0').to_string()
}

/// Wei as ether.
pub(crate) fn ether(wei: u128) -> String {
    format(wei, 18)
}

/// Wei per gas as gwei.
pub(crate) fn gwei(wei: u128) -> String {
    format(wei, 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amounts_read_as_a_person_writes_them() {
        assert_eq!(ether(1_000_000_000_000_000_000), "1");
        assert_eq!(ether(1_500_000_000_000_000), "0.0015");
        assert_eq!(ether(1), "0.000000000000000001");
        assert_eq!(gwei(2_100_000_000), "2.1");
        assert_eq!(ether(0), "0");
    }
}
