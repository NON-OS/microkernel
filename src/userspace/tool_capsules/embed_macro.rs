// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The `tool_capsule!` macro: bake one tool's signed artifacts into the image.

/// Build a `ToolCapsule` from its endpoint identity and its binary name. The ELF
/// path is given explicitly; the cert, manifest, and attestation trailer are
/// found by binary name under the trust directory. All paths resolve relative to
/// the invoking file, which is the registry.
macro_rules! tool_capsule {
    ($name:literal, $port:expr, $reply:literal, $reply_port:expr, $elf:expr, $bin:literal) => {
        tool_capsule!($name, $port, $reply, $reply_port, $elf, $bin, super::spec::SANDBOX_CAPS)
    };
    ($name:literal, $port:expr, $reply:literal, $reply_port:expr, $elf:expr, $bin:literal, $caps:expr) => {{
        const ELF: &[u8] = include_bytes!($elf);
        const CERT: &[u8] = $crate::baked_trust!(concat!("capsules/", $bin, ".nonos_id_cert.bin"));
        const MANIFEST: &[u8] = $crate::baked_trust!(concat!("capsules/", $bin, ".manifest.bin"));
        const ATTESTATION: &[u8] =
            $crate::baked_trust!(concat!("capsules/", $bin, ".zk_trailer.bin"));
        // A tool built in with an empty file never compiles (B-02).
        const _: () = assert!(
            !ELF.is_empty() && !CERT.is_empty() && !MANIFEST.is_empty() && !ATTESTATION.is_empty(),
            concat!("tool ", $bin, " built in with an empty image")
        );
        super::spec::ToolCapsule {
            caps: $caps,
            name: $name,
            service_port: $port,
            reply_inbox: $reply,
            reply_port: $reply_port,
            elf: ELF,
            cert: CERT,
            manifest: MANIFEST,
            attestation: ATTESTATION,
        }
    }};
}
