# Rust to Lean extraction

This directory makes the refinement between the Lean specification and the
kernel machine-checked instead of nominal. The pipeline lowers the real kernel
source to Lean through the compiler's own MIR and proves the security semantics
about the result:

```
src/capabilities/bits.rs          the code that ships
        |  rustc MIR, via Charon (#[path] include, never a copy)
        v
caps/caps.llbc                    the borrow-checked IR of that exact code
        |  Aeneas, Lean backend
        v
lean/NonosExtraction/Caps.lean    the same functions as Lean definitions
        |  lean/NonosExtraction/Refinement.lean
        v
Grants / capsOf semantics         the objects the lattice theorems quantify over
```

`Caps.lean` is generated, never edited; regenerating it from an unchanged kernel
is byte-stable, and any kernel change shows up as a diff here. The theorems in
`Refinement.lean` prove the extracted `has_capability`, `add_capability`, and
`remove_capability` are total and compute exactly the `capsOf` denotation from
`verification/lean/Nonos/CapabilityBits.lean`, so every lattice theorem proven
there now holds of the extracted-from-real-Rust definitions by transitivity.

## Parts

- [Scope, honestly stated](extraction-scope-honestly-stated.md)
- [Trusted base and pins; Reproduce](extraction-trusted-base-and.md)
