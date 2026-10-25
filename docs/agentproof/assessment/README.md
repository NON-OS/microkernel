# The AgentProof assessment

A fixed-fee engagement, $18,000, about two weeks. We run your AI agent, or code an AI wrote, in
NONOS boxes and give you a written account of everything it tried that a box refused: what it
wrote, where it reached for the network, what it started, by kind and count, with examples. You
get a policy for each agent you can keep using, and every log behind the report.

It is the first step before a pilot. Its findings come from the same enforcement a pilot or a
licence runs on, on a booted NONOS 0.9.3 image, never from a simulation.

## What you get

- **The report** ([template](report-template.md)): for each agent and task, the refusals by
  kind with examples, what the agent was allowed, and what each finding means for your risk.
- **A policy per agent**, as a `nonos-box 1` file with its hash, checked by
  `agentproof policy check` ([format](../policy-format.md)), ready for the pilot.
- **The evidence**: each run's serial log (its `[BOX]` line or `[BOX-MISSING]` line), the
  agent's own output, screenshots, and the exact commands, so any run can be repeated.
- **A read-out**: an hour with your team on the findings and on what a pilot would cover.

## What it covers in this release (0.9.3)

Read this before you scope. The assessment claims only what the code does today.

- **Agents that run as scripts.** A program runs in a box only if NONOS can prove it: the
  programs in the image's Linux tree (`python3` 3.12 with its standard library, `sh` and the
  BusyBox tools, `perl`, `lua`, `tclsh`, `qjs`, `jq` and others) and packages from the store. An
  agent written for one of these runs as it is; an agent that needs a binary or a library NONOS
  does not carry needs us to package and sign it first, which is a pilot item, not this one.
- **Network only to destinations you list, by IPv4 address.** A box may name destinations;
  the agent reaches those over the anonymity network and nothing else, and the report names
  each one it reached. A name lookup from inside a box does not yet return in this release.
  Our lab boots without a route to the mixnet, so an agent that calls a hosted model is run
  against that model's recorded replies; we see what it does with them, and which
  destinations it tried.
- **x86_64, in QEMU with the Intel IOMMU enforcing**, which is where every box run on these
  pages was booted. Running on your own hardware needs that machine booted first.
- **The report is not a signed receipt.** Its counts are what `capsule_linux` refused, sent
  to the Terminal on a key the program never sees, so the program cannot forge or hide them.
  The kernel's sealed record, a TPM quote and an offline verifier come with receipt v0 in
  0.9.4; until then nothing in the report is signed.

## How it runs

[The steps](steps.md): scoping, policies, the runs, analysis, the report.
[What we measure](measures.md): every number in the report and the code that makes it.

## What it is not

Not a review of your model's answers, and not a penetration test of your network. It answers
one question: when your agent runs, what does it try to do beyond what you meant it to?
