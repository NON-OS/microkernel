#!/usr/bin/env bash
# Give the runner user /dev/kvm for the QEMU boot jobs, and print the host
# CPU first: kernel bugs here have been vendor-specific (#524 reproduced only
# on AuthenticAMD), so a boot verdict is only readable next to the CPU.
#
# Usage: enable-kvm.sh --require       fail when the runner offers no KVM
#        enable-kvm.sh --allow-tcg     fall back to TCG with a longer deadline
#
# With --allow-tcg, BOOT_TIMEOUT (seconds) goes to $GITHUB_ENV: 300 under KVM,
# 900 under TCG. NONOS_FORCE_TCG=true removes /dev/kvm first, so the TCG path
# can be exercised on demand.

set -euo pipefail

mode="${1:?usage: enable-kvm.sh --require|--allow-tcg}"

awk -F: '/^vendor_id|^model name|^cpu family|^model[[:space:]]/ {print $1": "$2}' /proc/cpuinfo | sort -u

echo 'KERNEL=="kvm", GROUP="kvm", MODE="0666", OPTIONS+="static_node=kvm"' \
    | sudo tee /etc/udev/rules.d/99-kvm4all.rules >/dev/null
sudo udevadm control --reload-rules
sudo udevadm trigger --name-match=kvm

if [ "${NONOS_FORCE_TCG:-false}" = "true" ]; then
    sudo rm -f /dev/kvm
    echo "::warning::TCG requested; /dev/kvm removed for this job"
fi

summary() {
    if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then echo "$1" >> "${GITHUB_STEP_SUMMARY}"; fi
}

case "${mode}" in
    --require)
        if ! test -w /dev/kvm; then
            echo "::error::this runner offers no KVM, and this job needs it"
            exit 1
        fi
        summary "accel: kvm"
        ;;
    --allow-tcg)
        if test -w /dev/kvm; then
            timeout=300 accel="kvm (deadline 300 s)"
        else
            echo "::warning::this runner offers no KVM; booting under TCG instead"
            timeout=900 accel="TCG, no KVM on this runner (deadline 900 s)"
        fi
        if [ -n "${GITHUB_ENV:-}" ]; then echo "BOOT_TIMEOUT=${timeout}" >> "${GITHUB_ENV}"; fi
        summary "accel: ${accel}"
        ;;
    *)
        echo "enable-kvm: unknown mode ${mode}" >&2
        exit 2
        ;;
esac
