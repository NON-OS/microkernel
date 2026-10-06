//! The card a measured run leaves behind, written to a file so the figure can be quoted and
//! diffed. It holds timings, sizes and the instance shape, never a key, address, amount or note.

use super::BenchReport;
use core::fmt::Write;

/// The card for one measured run. The note says where it ran, which only the shell knows.
pub fn card(report: &BenchReport, note: &str) -> String {
    let mut out = String::new();
    let verified = if report.verified { "true" } else { "FALSE" };
    // A card with a missing line is worse than none, so a failed write returns an empty card.
    let written = write!(
        out,
        "proved: {} s ({} ms)\nverified: {} ms, {}\nproof: {} bytes\nsoundness: {} bits, \
         {} queries at grind {}\nbuilt: {} ms\n",
        crate::ffi::format_seconds(report.proved_ms),
        report.proved_ms,
        report.verified_ms,
        verified,
        report.proof_bytes,
        report.shape.security_bits,
        report.shape.queries,
        report.shape.grind_bits,
        report.built_ms,
    );
    if written.is_err() {
        return String::new();
    }
    if let Some(kib) = report.peak_kib {
        if writeln!(out, "peak: {kib} KiB").is_err() {
            return String::new();
        }
    }
    if shape(&mut out, report).is_err() || writeln!(out, "where: {note}").is_err() {
        return String::new();
    }
    out
}

/// The measured instance, so a fast run on a smaller circuit is not quoted as a phone result.
fn shape(out: &mut String, report: &BenchReport) -> core::fmt::Result {
    write!(
        out,
        "rows: 2^{}\ncolumns: {}\ndegree: {}\nperiodic columns: {}\npublic words: {}\n",
        report.shape.log_trace_len,
        report.shape.trace_width,
        report.shape.constraint_degree,
        report.shape.periodic_columns,
        report.shape.public_words,
    )
}
