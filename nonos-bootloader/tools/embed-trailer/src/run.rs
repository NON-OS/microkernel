// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::fs;

use anyhow::{bail, Context, Result};
use embed_trailer::*;

use crate::summary_image::print_image_summary;

/// Carry the enrolled trailer into the image. The trailer is bound to this
/// exact kernel measurement, and the bootloader folds it against the same
/// BLAKE3 over the same kernel bytes. Anything that is not a path trailer is
/// refused here rather than shipped for the bootloader to refuse.
pub fn run_attestation(args: &Args) -> Result<()> {
    let kernel = load_signed_kernel(&args.input)?;
    let kernel_hash = compute_kernel_hash(&kernel.kernel_bytes);
    let trailer = fs::read(&args.proof_file)
        .with_context(|| format!("Failed to read proof file: {}", args.proof_file.display()))?;
    let path_only = args.path_only && trailer.starts_with(&nonos_attest_path::MAGIC);
    if !path_only && !trailer.starts_with(&nonos_attest_path::MAGIC_V4) {
        bail!("{} is not a v4 self-attestation trailer", args.proof_file.display());
    }
    if args.verbose {
        println!("Signed kernel: {} bytes", kernel.raw_bytes.len());
        println!("Kernel code: {} bytes", kernel.kernel_bytes.len());
        println!("Kernel BLAKE3: {}", hex::encode(kernel_hash));
        println!("Trailer: {} bytes", trailer.len());
    }
    let image = assemble_attested_image(&kernel, trailer);
    fs::write(&args.output, &image.data)
        .with_context(|| format!("Failed to write: {}", args.output.display()))?;
    print_image_summary(args, &image);
    Ok(())
}
