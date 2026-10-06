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
