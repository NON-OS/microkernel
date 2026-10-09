//! What a scan cost and what it found.

/// Scan counters. Tag hits over outputs is the view tag's real selectivity on live data.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ScanStats {
    /// Outputs examined.
    pub outputs: u64,
    /// Key agreements performed, one per output.
    pub agreements: u64,
    /// Outputs whose view tag matched, so an AEAD open was attempted.
    pub tag_hits: u64,
    /// Ciphertexts that opened and committed to what they claimed.
    pub notes_found: u64,
    /// Ciphertexts that opened but did not match their commitment or carry a spendable key.
    pub rejected: u64,
}

impl ScanStats {
    /// Fold another pass in, so a multi page sync reports one set of numbers.
    pub fn merge(&mut self, other: &ScanStats) {
        self.outputs = self.outputs.saturating_add(other.outputs);
        self.agreements = self.agreements.saturating_add(other.agreements);
        self.tag_hits = self.tag_hits.saturating_add(other.tag_hits);
        self.notes_found = self.notes_found.saturating_add(other.notes_found);
        self.rejected = self.rejected.saturating_add(other.rejected);
    }
}
