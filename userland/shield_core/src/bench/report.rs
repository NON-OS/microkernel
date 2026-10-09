//! One measured proof. Every field is observed on the device, and the shape names the instance.

/// Which circuit and point a measured proof was made at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shape {
    pub log_trace_len: u32,
    pub trace_width: u32,
    pub constraint_degree: u32,
    pub periodic_columns: u32,
    pub public_words: u32,
    pub queries: u32,
    pub grind_bits: u32,
    pub extra_blowup_bits: u32,
    pub security_bits: u32,
}

/// One measured proof. Times are milliseconds on the device, and a false `verified` voids them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BenchReport {
    pub built_ms: u64,
    pub proved_ms: u64,
    pub verified_ms: u64,
    pub verified: bool,
    pub proof_bytes: u64,
    pub peak_kib: Option<u64>,
    pub shape: Shape,
    /// The parameter id read from the proof's own header.
    pub params_id: String,
    /// Whether that id is the launch point's, the proof the pool verifies.
    pub launch_point: bool,
    /// Where the proof and its public words were written for export.
    pub proof_path: Option<String>,
}
