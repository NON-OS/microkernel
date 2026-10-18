# uart_proofs

Host tests for the pure part of the aarch64 console UART, mounted by `#[path]`
from `src/arch/aarch64/uart/`:

- `divisor.rs`: the PL011 integer and fractional divisors, the 16550 divisor
  latch and the BCM2835 mini-UART baud register. Each is pinned at the
  values the boards here use (QEMU virt's 24 MHz PL011, the Pi's 48 MHz
  PL011 and 250/500 MHz mini-UART, a 1.8432 MHz 16550), and swept to show
  the chosen divisor is the nearest one and that a rate no divisor can make
  is refused rather than truncated.

Run with `RUST_TEST_THREADS=1 cargo test --release --config
profile.release.overflow-checks=true` from this directory. `nix flake check`
runs the same as `proofs-uart_proofs`, with clippy.
