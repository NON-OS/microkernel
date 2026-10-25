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
| `arith_tests`, `unary_tests`, `logic_tests` | every integer and logical operator against its definition, at 64 and 32 bits |
| `convert_tests`, `text_tests` | implicit and explicit conversions; Concatenate, ConcatenateResTemplate, Mid, SizeOf |
| `store_tests`, `ref_tests`, `debug_tests` | Store and CopyObject rules; RefOf, DerefOf, CondRefOf, Index, ObjectType |
| `control_tests`, `call_tests` | control flow, arguments and locals, calls, method-made objects removed, serialized recursion |
| `budget_tests`, `budget_limit_tests` | a loop runs exactly to its step budget; recursion, allocation and waiting stopped |
| `sync_tests`, `event_tests`, `host_tests` | mutex nesting and sync order; events; Notify's queue; Fatal, Timer, Sleep, Stall |
| `osi_tests`, `osi_ladder_tests` | every published Windows string answered, nothing else, capping at a release, the firmware ladder |
| `predefined_tests` | \_OS, \_OSI, \_REV; load-time code run by the interpreter |
| `generated_tests` | 3000 random expressions against a model of the operators, at both widths |
| `region_math_tests` | access widths, the units a field touches, UpdateRule merges, bit strings, Global Lock steps |
| `field_io_tests`, `field_rule_tests` | fields through a modelled region; byte and AnyAcc widths; buffers; bounds; handlers; UpdateRule; IndexField; BankField |
| `pci_reg_tests` | a PCI_Config region's segment, bus and _ADR path; _REG on connect, a faulting _REG reported |
| `buffer_field_tests` | _OSC-style buffer fields on an argument, CreateField, bounds, load-time buffer fields |
| `lock_tests` | the Global Lock around a Lock field and \_GL, waiting for firmware, GBL_RLS, release on fault |
| `transaction_tests` | an SMBus write's response, GPIO pins through a Connection, DataTableRegion |
| `load_op_tests`, `load_table_tests` | Load from a buffer and a region, LoadTable with its parameter, Unload, a method region from its argument |
| `field_mutation_tests` | thousands of mutated field programs with every space connected: a value or a fault, the Global Lock given back |
| `exec_mutation_tests` | thousands of mutated method tables loaded and every method run: a value or a fault, never a panic |
| `mutation_tests` | thousands of mutated and random tables end in a tree or a refusal, never a panic |

`src/tests/catalog*` holds one instance of every opcode; `src/flat.rs` is a
namespace stand-in that knows methods by their last NameSeg; `src/consts.rs`
is a load-time evaluator that understands only integer constants; `src/probe.rs`
is a host that records what the interpreter asked of it and models the
hardware a field reaches; `src/rig.rs` keeps a loaded table and its probe
together.

Run: `RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true`.
