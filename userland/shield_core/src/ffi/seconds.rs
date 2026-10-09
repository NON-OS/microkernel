//! A duration as a screen shows it: seconds with one decimal, a point for
//! the decimal and no grouping, whatever the phone's language. Left to a
//! shell, 61212 ms printed in an Italian locale as "61.212", which reads as
//! sixty-one milliseconds everywhere else.

/// `ms` as seconds to one decimal, rounded down: 61212 is "61.2".
pub fn format_seconds(ms: u64) -> String {
    let whole = ms.checked_div(1_000).unwrap_or(0);
    let tenth = ms.checked_rem(1_000).and_then(|r| r.checked_div(100)).unwrap_or(0);
    format!("{whole}.{tenth}")
}

#[cfg(test)]
mod tests {
    use super::format_seconds;

    #[test]
    fn a_proof_time_reads_the_same_in_every_language() {
        assert_eq!(format_seconds(61_212), "61.2");
        assert_eq!(format_seconds(52_240), "52.2");
        assert_eq!(format_seconds(999), "0.9");
        assert_eq!(format_seconds(0), "0.0");
    }
}
