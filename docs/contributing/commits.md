# Commits

How a NONOS commit is written: the subject line, the body, and what a commit leaves out.

Everything below is read from the history of `main` at this commit. To see the same commits:

```
git log --oneline -50
```

## The subject line

- It starts with the area the change touches and a colon. The area is the component as the tree names it, usually in lower case: `nvme:`, `iommu:`, `futex:`, `rtl8821ce:`, `linux sh:`, `static checks:`, `docs:`. Two areas are joined with "and", as in `e1000e and igc:`.
- After the colon comes one full sentence that says what is true once the change is in, in the present tense. The effect or the reason often follows after "so".
- It has no full stop. In the last 50 commits subjects run from 87 to 180 characters; a sentence that says what changed matters more than a short line.
- Every one of the last 200 commits on `main` ends its subject with `[skip ci]`. GitHub Actions does not start push and pull request workflows for a commit whose message carries that marker, so CI did not run on those commits; their bodies record what was run by hand.

Three subjects from `main`:

```
nvme: a controller that stops answering right after taking the host memory buffer fails that attempt, and the next one runs the drive without it [skip ci]
futex: a waiter that returns because the word changed passes on a wake a waker already spent on it, so the waiter behind it is not left to its timeout [skip ci]
rtl8153: the refusals of an RTL8156 and an RTL8153C name the chip instead of admitting unsupported work, so the stubs gate passes [skip ci]
```

## The body

The body says what was wrong, what the change does, and how it was checked, in that order, wrapped at about 72 columns.

1. What was wrong, and when it is known, the commit that introduced it, by its short hash.
2. What the change does: the files, the behaviour, and the log line it prints if it prints one.
3. A last paragraph that starts with `Verified:` and lists what ran and its numbers: the proof crate and its test count, clippy, `tools/nix/inputs.py --check`, a build for the capsule target. When something was not run, the paragraph ends by naming it, as in "Not verified on hardware." or "Not verified: a boot."

In the last 50 commits, 29 bodies carry a `Verified:` paragraph, 14 say what was not verified, and 23 name an earlier commit by its hash. One body in full:

```
rtl8153: the refusals of an RTL8156 and an RTL8153C name the chip instead of admitting unsupported work, so the stubs gate passes [skip ci]

scripts/check_stubs.py (static-hygiene) flagged version.rs:45 and :47:
the version check refused an RTL8156 and an RTL8153C with "not
implemented". Neither is work this driver leaves undone: an RTL8156 is
another chip (r8156_init), and the RTL8153C needs r8153c_init, which
this driver does not carry. The refusals now say so: "an RTL8156, not
an RTL8153" and "an RTL8153C, whose init this driver lacks". The check
is unchanged; behaviour is unchanged, only the log text.

Verified: check_stubs.py reports no rtl8153 site; rtl8153_proofs 27
pass; cargo clippy -D warnings clean on rtl8153_proofs (--tests) and on
capsule_driver_rtl8153 for x86_64-nonos-user; inputs.py --check passes.
```

## What a commit leaves out

- Trailers. None of the last 200 commits on `main` carries a sign-off or a co-author line; the only lines git reads as trailers are some of their `Verified:` paragraphs. Keep to that.
- Key material of any kind. The flake build never holds a key; signing happens in the [seal](../overview/glossary.md#seal), a separate step (`seal`, `Makefile:11-14`). [Signing and publisher keys](../userland/signing-and-publisher-keys.md) explains where keys live.
- Unrelated changes. One concern per commit; a formatting pass over files you did not otherwise change goes in a commit of its own, or nowhere.
