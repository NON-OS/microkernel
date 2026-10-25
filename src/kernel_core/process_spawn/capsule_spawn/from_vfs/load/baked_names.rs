// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

//! The service names this kernel image's own spawn sites declare (K18),
//! each with the namespace its capsule is signed under. A capsule loaded at
//! run time may declare one, or a name under one, only when it is signed
//! under that same namespace: the spawn site's identity, which only the
//! NONOS root signs. The reservation holds whether or not the owner runs,
//! and ends only when a later signed image drops the spawn site. An empty
//! namespace means no load may take the name. scripts/check_baked_names.py
//! holds the lists to the spawn sites; baked_apps.rs has the apps.

/// Families the kernel trusts by name: the held drivers (held_table.rs) and
/// the network stack. A member is `systems.nonos.<its name>`'s alone.
pub(crate) const FAMILIES: &[&str] = &["driver", "net"];

/// The core services, the desktop and the proof services.
pub(crate) const CORE: &[(&str, &str)] = &[
    ("acpi", "systems.nonos.acpi"),
    ("attest", "systems.nonos.attest"),
    ("audio.server", "systems.nonos.audio.server"),
    ("clipboard", "systems.nonos.clipboard"),
    ("compositor", "systems.nonos.compositor"),
    ("crypto_pool", "systems.nonos.crypto"),
    ("desktop_shell", "systems.nonos.desktop_shell"),
    ("entropy_pool", "systems.nonos.entropy"),
    ("image_codec", "systems.nonos.image_codec"),
    ("input_router", "systems.nonos.input_router"),
    ("installer", "systems.nonos.installer"),
    ("keyring", "systems.nonos.keyring"),
    ("login", "systems.nonos.login"),
    ("market.index", "systems.nonos.market"),
    ("nonos.prove.fetch", "systems.nonos.prove.fetch"),
    ("nonos.shield", "systems.nonos.shield"),
    ("nox_settle", "systems.nonos.nox_settle"),
    ("payment", "systems.nonos.payment"),
    ("policy", "systems.nonos.policy"),
    ("power", "systems.nonos.power"),
    ("proof_io", "systems.nonos.proof_io"),
    ("ramfs", "systems.nonos.ramfs"),
    ("shield_vectors", "systems.nonos.shield_vectors"),
    ("std_proof", "systems.nonos.std_proof"),
    ("test.attack", "systems.nonos.test.attack"),
    ("tokio_smoke", "systems.nonos.tokio_smoke"),
    ("toolkit", "systems.nonos.toolkit"),
    ("vfs_pool", "systems.nonos.vfs"),
    ("wallpaper", "systems.nonos.wallpaper"),
    ("wallpaper_catalog", "systems.nonos.wallpaper_catalog"),
    ("wm", "systems.nonos.wm"),
];
