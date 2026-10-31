#!/usr/bin/env bash
# Kani over the proof crates whose harnesses model-check kernel and driver
# code, the trailer parser every gate shares (parse_v4 and the v3 path), and
# the loader's rollback floor read and its rule. A crate missing from the
# tree fails the run rather than being passed over.
#
# Usage: kani-proof-crates.sh

set -euo pipefail
cd "$(dirname "$0")/.."

crates=(
    userland/kernel_proofs userland/net_proofs userland/driver_proofs userland/usb_proofs
    userland/nvme_proofs userland/usb_msc_proofs userland/virtio_blk_proofs
    userland/virtio_net_proofs userland/e1000_proofs userland/rtl8139_proofs
    userland/xhci_proofs userland/sync_proofs userland/mechanism_proofs
    userland/wallet_proofs userland/arch_paging_proofs
    nonos-attest-path
    nonos-bootloader/boot_proofs
)
for crate in "${crates[@]}"; do
    echo "::group::cargo kani ${crate}"
    (cd "${crate}" && cargo kani --output-format terse)
    echo "::endgroup::"
done
