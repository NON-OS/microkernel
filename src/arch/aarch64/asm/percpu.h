/* Offsets into this CPU's per-CPU block (smp::percpu::PerCpuData), which
 * TPIDR_EL1 names on every CPU once boot installs it. layout.rs asserts the
 * Rust side and the proofs hold the two equal.
 *
 * The pointer-authentication words: the IA key of the process this CPU next
 * enters EL0 with, the kernel's, and whether keys are switched at all. That
 * last word, not TPIDR_EL1 being set, is what lets the vectors touch the key
 * registers, which a CPU without pointer authentication does not have. */

#define PERCPU_PAC_USER 136
#define PERCPU_PAC_KERNEL 152
#define PERCPU_PAC_ON 168
