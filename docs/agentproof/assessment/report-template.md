# AgentProof assessment report: <customer>

Date: <date> · NONOS image: <version, commit> · Platform: <QEMU or machine>, IOMMU
<enforcing=1, unconfined grants=0> · Assessor: <name>

## Summary

<Three sentences: which agents, what they were meant to do, and the most important thing they
tried beyond it.>

| Agent | Task | Box | Refused | Worst finding |
|---|---|---|---|---|
| <agent> | <task> | <box word> | <n> | <one line> |

## Scope

- Agents: <name, interpreter, script, version>.
- Tasks: <each, with its inputs and any recorded model replies>.
- Not in scope: <anything excluded and why>.

## Policies

For each agent, the box it ran in:

```
<the nonos-box 1 file>
```

- Word: `<allow:...>`. Hash: `<64 hex>`.
- In plain words: <`agentproof policy check` output>.

## Findings

One section per agent and task.

### <agent> — <task> — <pass: as meant | pushed | wider box>

- Command: `linux --box <box> <interpreter> - < <script>`
- Report: started <yes|no>, exit <n>, refused <n>.

| Kind | Count | Examples (first three) | What it means |
|---|---|---|---|
| write | <n> | <paths> | <expected, overreach or compromise> |
| network | <n> | <what> | <...> |

- What the agent was allowed and did not need: <...>.
- Recommendation: <tighten the box, change the agent, or accept>.

## Runs that failed

<Any run with no `[BOX]` line or with `[BOX-MISSING]`: why, and its rerun.>

## What this report does and does not prove

- Every count is what `capsule_linux` refused in the box named, on the image named, sent to the
  Terminal on a key the program never saw.
- It is not signed: signed receipts with the kernel's sealed record and a TPM quote are planned
  for NONOS 0.9.4. No box had a network.

## Evidence

<A list of each run: serial log file, output file, screenshot, and the command, so it can be run
again on the same image.>
