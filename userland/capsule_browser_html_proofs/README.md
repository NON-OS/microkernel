# capsule_browser_html_proofs

Host test crate for the browser's HTML parser. It builds for the host and compiles the
tokenizer and tree builder from `userland/capsule_browser` through `#[path]`, so `cargo test`
runs the parser source the capsule ships against known answers and hostile input, with no
boot, no renderer and no network. It has no dependencies (`Cargo.toml`).

## What is under test

`src/browser/mod.rs` mounts two module trees from the capsule, unchanged:

- `../../../capsule_browser/src/browser/dom/mod.rs` as `browser::dom`: the tree builder and the
  node arena it fills.
- `../../../capsule_browser/src/browser/html/mod.rs` as `browser::html`: the input decoder, the
  tokenizer and the character reference table.

Two local helpers read results back: `src/shape.rs` (markup under an element, elements by
name, nesting depth, arena invariants) and `src/tokens.rs` (tokens as short strings, read from
a chosen tokenizer state).

## What the tests check

Thirteen test modules are listed in `src/lib.rs`. The doc comments cite WHATWG HTML section
numbers for the behaviour they target.

- `tokenizer_tests.rs`, `raw_text_tests.rs`: tag and attribute states, RCDATA, RAWTEXT,
  script data escapes, PLAINTEXT and CDATA (for example
  `names_are_lowercased_and_the_first_duplicate_wins`,
  `script_data_escapes_hide_a_nested_end_tag`).
- `tree_tests.rs`, `table_tests.rs`, `foreign_tests.rs`, `fragment_tests.rs`: implied
  elements and end tags, the adoption agency, content moved out of a table, SVG and MathML namespaces,
  and fragment parsing (`misnested_formatting_is_adopted_and_reopened`,
  `stray_content_is_fostered_before_the_table`, `inner_html_never_adds_html_head_or_body`).
- `entity_tests.rs`, `legacy_tests.rs`: named and numeric character references, and where a
  reference is left as written (`the_longest_name_in_the_table_wins`).
- `input_tests.rs`, `content_tests.rs`: UTF-8 decoding with replacement, BOM and newline
  handling, and a value over one mebibyte being dropped whole
  (`invalid_utf8_becomes_one_replacement_per_maximal_subpart`).
- `budget_tests.rs`, `hostile_tests.rs`, `fuzz_tests.rs`: attribute and node caps, the depth
  cap, deep and misnested input, and a fixed-seed random markup pass held to the arena
  invariants (`the_node_cap_stops_growth_and_says_so`,
  `random_markup_always_builds_a_sound_tree`).

## Running

```sh
cd userland/capsule_browser_html_proofs
cargo test --release
```

At the time of writing this runs 43 tests. CI runs it through `nix flake check`, which
`tools/nix/checks.nix` builds over every `userland/*_proofs` crate with a `Cargo.lock`: the
tests in release mode with overflow checks on, then clippy over all targets with
`-D warnings`.

## Not covered

CSS, layout, paint, scripting, fetch and TLS are not mounted here (several of those are in
`userland/capsule_browser_proofs`). The fuzz pass is fixed-seed and small, not
coverage-guided. Nothing in this crate runs the parser inside the capsule or on target.
