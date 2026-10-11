# nonos_aml_proofs

Host proofs for `nonos_aml`, the AML interpreter `capsule_acpi` runs. The
shipping sources are mounted by `#[path]`; every fixture is built by the AML
encoder in `src/enc`, written from chapter 20 of the ACPI 6.5 specification,
so no table enters the tree as an opaque blob.

| Tests | What they hold |
|---|---|
| `opcode_tests` | every object in the catalogue parses to its node, offset and operand count; Else joins its If |
| `coverage_tests` | every byte value on both opcode pages that section 20.3 does not define is refused |
| `pkg_length_tests` | each PkgLength size at its bounds, reserved bits, too-short and past-parent lengths |
| `name_tests` | each NameString form, NullName, malformed and truncated names |
| `position_tests` | statements, constants, Debug and names only where the grammar puts them |
| `arity_tests` | calls take the arguments their Method or External declares; a SuperName never calls |
| `field_tests` | each FieldElement kind, and malformed ones |
| `package_tests` | package elements, buffers, a package overrunning its parent |
| `truncation_tests` | every cut of the catalogue and of a method body is refused unless it falls between objects |
| `limit_tests` | depth, node count and table size, each exact at its edge |
| `header_tests` | the table header, checksum and length field |
| `search_tests` | the predefined objects; the search rules for a lone NameSeg and exact paths otherwise |
| `load_tests`, `load_order_tests` | every named object becomes its kind; SSDTs add to the DSDT; table order; a refused table adds nothing |
| `redefine_tests`, `alias_note_tests` | the first definition stands, containers keep both bodies, predefined objects cannot be replaced, missing targets are noted |
| `field_unit_tests` | field units laid out from Field, IndexField and BankField lists; a layout past 2^32 bits stops |
| `load_exec_tests` | a load-time If loads the branch it chooses; other statements run as the table loads |
| `load_arity_tests` | calls counted from earlier tables, by the search rules, and from a method's own scope |
| `mutation_tests` | thousands of mutated and random tables end in a tree or a refusal, never a panic |

`src/tests/catalog*` holds one instance of every opcode; `src/flat.rs` is a
namespace stand-in that knows methods by their last NameSeg; `src/consts.rs`
is a load-time evaluator that understands only integer constants.

Run: `RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true`.
