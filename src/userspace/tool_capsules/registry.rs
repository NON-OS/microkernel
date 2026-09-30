// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The list of embedded tool capsules. The block between the generated markers
//! is written by `tools/nonos-app` from `userland/apps.list`, the single
//! source of truth. Adding a tool is `nonos-app add <crate>`, not a hand edit.

extern crate alloc;

#[cfg(feature = "nonos-tool-capsules")]
use alloc::vec;
use alloc::vec::Vec;

use super::spec::ToolCapsule;

/// Every embedded tool capsule, generated from `userland/apps.list`. Off the
/// `nonos-tool-capsules` feature (core builds that do not cross-compile the
/// tool binaries) the list is empty, so nothing is `include_bytes`d.
#[cfg(not(feature = "nonos-tool-capsules"))]
pub(super) fn embedded_tools() -> Vec<ToolCapsule> {
    Vec::new()
}

#[cfg(feature = "nonos-tool-capsules")]
pub(super) fn embedded_tools() -> Vec<ToolCapsule> {
    vec![
        // nonos-app:begin (generated; do not edit by hand)
        tool_capsule!(
            "tool.grex",
            4900,
            "endpoint.tool.grex.reply",
            4901,
            "../../../target/upstream-grex/bin/grex",
            "grex"
        ),
        tool_capsule!(
            "tool.dotenv-linter",
            4902,
            "endpoint.tool.dotenv-linter.reply",
            4903,
            "../../../target/upstream-dotenv-linter/bin/dotenv-linter",
            "dotenv-linter"
        ),
        tool_capsule!(
            "tool.pastel",
            4904,
            "endpoint.tool.pastel.reply",
            4905,
            "../../../target/upstream-pastel/bin/pastel",
            "pastel"
        ),
        tool_capsule!(
            "tool.jsonxf",
            4906,
            "endpoint.tool.jsonxf.reply",
            4907,
            "../../../target/upstream-jsonxf/bin/jsonxf",
            "jsonxf"
        ),
        tool_capsule!(
            "tool.tokei",
            4910,
            "endpoint.tool.tokei.reply",
            4911,
            "../../../target/upstream-tokei/bin/tokei",
            "tokei"
        ),
        tool_capsule!(
            "tool.huniq",
            4912,
            "endpoint.tool.huniq.reply",
            4913,
            "../../../target/upstream-huniq/bin/huniq",
            "huniq"
        ),
        tool_capsule!(
            "tool.csview",
            4914,
            "endpoint.tool.csview.reply",
            4915,
            "../../../target/upstream-csview/bin/csview",
            "csview"
        ),
        // nonos-app:end
        /*
         * First-party, hand-registered: the command-line installer. Not a
         * crates.io tool and not in apps.list, so it sits outside the
         * generated block, and it declares the authority the window version
         * has: disks, the boot image, entropy, the attestation verdict, and
         * the reboot at the end.
         */
        #[cfg(feature = "nonos-capsule-install-cli")]
        tool_capsule!(
            "tool.install",
            4934,
            "endpoint.tool.install.reply",
            4935,
            concat!(
                "../../../userland/tool_install/target/",
                env!("NONOS_USER_TARGET"),
                "/release/install-cli"
            ),
            "install-cli",
            crate::userspace::capsule_install::CLI_CAPS
        ),
    ]
}
