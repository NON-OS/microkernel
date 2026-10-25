# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.


"""The rows dropped from the suite, each with its date and its reason."""

# Rows the suite once expected and nothing on the boot path can show. Each
# keeps its date and reason, so it is put back when a boot can show it, not
# forgotten.
DROPPED = {
    "kernel-replay": (
        "2026-10-24",
        "the loader's rollback floor needs a TPM counter, and the self-test boots have "
        "none: under QEMU the floor reads Unprotected and an older signed kernel would "
        "boot (nonos-bootloader/src/boot/crypto/rollback/floor.rs). The floor rule is "
        "proven on the host instead (boot_proofs floor_rule_tests, floor_seq_tests, "
        "floor_raise_tests)",
    ),
    "chainload": (
        "2026-10-24",
        "the loader has no chainload path to attack: its shell launcher starts no "
        "image (nonos-bootloader/src/boot/shell/launch.rs)",
    ),
}
