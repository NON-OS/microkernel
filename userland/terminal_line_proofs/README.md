# terminal_line_proofs

Host test crate for the pure parts of the NONOS terminal. It compiles files of
`userland/capsule_terminal` through `#[path]`, unchanged, and runs them against
known answers on the host, with no window and no kernel. It depends on
`nonos_vt` and `nonos_policy_proto` (`Cargo.toml`). The terminal is described
in [docs/handbook/apps/terminal.md](../../docs/handbook/apps/terminal.md).

## What is under test

From the terminal, as `src/lib.rs` mounts them: the line editor
(`term/line/`), history expansion, command suggestions, the flag parser, the
pipeline text filters, the `tree` renderer, `grep` highlighting, the line
editing a foreground program gets (`event/cooked.rs`, `cooked_kill.rs`),
pointer selection, the line's tokenizer, statement split and redirect plan
with what a tool may be given (`command/parse/`, `dispatch/statements.rs`,
`redirect.rs`, `tool_admit.rs`), the `help` pages and their layout, the tool list, the ANSI
palette, box-drawing strokes, and `direct_gate.rs` with `nonos_route_link`'s
`direct_only.rs`. `src/qwen.rs` mounts the `qwen` line handling, the tier
words and the kernel's own tier list (`src/userspace/capsule_linux/terminal/tier.rs`).

Three files from other capsules share the crate: the text editor's
`goto_line.rs` and `quick_open.rs`, and `net_core`'s serve-loop cadence.

## What the tests check

153 `#[test]` functions, among them:

- line editing, word kill, long lines and history `!` expansion;
- `<`, `>`, `>>` and `|` are operators against a word as well as apart,
  and text inside quotes; numbered streams are refused (`redirect_tests.rs`);
- the filters (`grep`, `sort`, `uniq`, `cut`, `nl`, `wc`, `head`, `tail`,
  `tac`, `rev`) and `tree` output;
- no `help` row is wider than the terminal, and every tool the terminal offers
  is registered;
- `qwen window` opens the named tier or the first, and the kernel is handed
  a tier word it allows;
- `ping`, `nslookup` and `pull` run only when Direct is the default network,
  for every value the policy store can hold, with the exact line each prints
  otherwise (`direct_gate_tests.rs`).

## Running

```sh
cd userland/terminal_line_proofs
cargo test --release
```

CI runs it through `nix flake check` (`tools/nix/checks.nix`), with overflow
checks on, then clippy over the library only: the crate is in the `lintLib`
list, whose tests are not lint-clean yet.

## Not covered

The window, the job pump, the network commands' I/O, `git` and every IPC call
are not mounted here.
