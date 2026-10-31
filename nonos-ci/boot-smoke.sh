#!/usr/bin/env bash
# The blocking boot gate, one step at a time, through the flake as a
# developer boots: scratch keys made on this runner, the qemu profile sealed
# with them, then two headless boots with a software TPM and no network.
#
#   keys   scratch keys for this runner only. The operator key stays the one
#          in the tree, so the seal keeps the index and catalogue the operator
#          signed and the enrollment that proves those exact capsules, rather
#          than re-proving all of them on the runner's two prover slots.
#   seal   nix run .#seal for the qemu profile
#   boot   boot from the image; the boot must reach the desktop's services
#          and the kernel must admit the loader it was measured with
#   usb    the same sealed stick on a USB port of the xHCI controller, as a
#          laptop boots it: the store loads through driver.xhci0 and
#          driver.usb_msc0, the path no other boot here takes
#
# Usage: boot-smoke.sh <keys|seal|boot|usb>
# boot and usb read BOOT_TIMEOUT (seconds; enable-kvm.sh --allow-tcg sets it).

set -euo pipefail
cd "$(dirname "$0")/.."

step="${1:?usage: boot-smoke.sh <keys|seal|boot|usb>}"
timeout="${BOOT_TIMEOUT:-300}"
# The lines every boot must print, in order.
common=(--expect 'Handoff OK' --expect 'Capsules spawned' --expect '\[VFS\] serving, store status ')

case "${step}" in
    keys)
        bash nonos-ci/setup-signing-key.sh
        NONOS_HOST_TOOLS="$(nix build .#host-tools --no-link --print-out-paths)" \
            NONOS_SCRATCH_KEYS_ONLY=1 NONOS_SCRATCH_OPERATOR=0 \
            nix develop --command bash nonos-ci/scratch-trust-bootstrap.sh
        ;;
    seal)
        nix run .#seal -- --profile qemu
        ;;
    boot)
        nix run .#qemu -- --profile qemu --tpm --headless --net off --timeout "${timeout}" \
            --serial target/qemu/serial.log "${common[@]}" \
            --expect '\[BOOT-ATTEST\] bootloader measured and enrolled'
        nix develop --command python3 nonos-ci/boot_attest_check.py --log target/qemu/serial.log --measured
        ;;
    usb)
        # The kernel takes a disk only at its first block I/O, which this boot
        # never makes, so the stick is not asked to bind here. Several CPUs
        # write the console at once and lines can interleave, so the driver's
        # start matches any one of the records it leaves.
        nix run .#qemu -- --profile qemu --tpm --stick --usb --headless --net off --timeout "${timeout}" \
            --serial target/qemu/serial-usb.log "${common[@]}" \
            --expect 'name=driver\.usb_msc0|runqueue_ok:driver\.usb_msc0|\[DRIVER-USB-MSC\] capsule spawned'
        ;;
    *)
        echo "boot-smoke: unknown step ${step}" >&2
        exit 2
        ;;
esac
