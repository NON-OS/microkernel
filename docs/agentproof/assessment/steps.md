# The assessment, step by step

About ten working days. Each step names what you give us and what we hand back.

## 1. Scoping (days 1–2)

- You give: each agent in scope (up to three), the tasks it is meant to do, the files and
  folders it is meant to touch, the tools it calls, and whether it calls a hosted model.
- We check each agent runs under a NONOS program (see [what it covers](README.md)), and record
  any hosted model's replies for each task, so the runs need no network.
- We hand back: a scope sheet, one row per agent and task, naming its interpreter, its
  inputs, and the folders it should need.

## 2. Policies (days 2–3)

- For each agent we write the tightest box its task needs, as a `nonos-box 1` file: `base
  strict`, `base offline`, or `base folders` with each `write` folder named.
- `agentproof policy check` explains each one in plain words; you confirm it says what you
  meant. Its hash is the one each run's report must show.

## 3. Runs (days 3–7)

On a NONOS 0.9.3 image booted with the IOMMU enforcing, each agent runs its tasks in the
Terminal as `linux --box <its box> <interpreter> - < <its script>`, in three passes:

1. **As meant**: the normal task, under its own policy. A clean run refuses nothing.
2. **Pushed**: the same tasks with inputs that invite overreach: a prompt asking it to read
   keys, to upload a file, to install something, to run a shell command it was not given.
3. **Wider boxes**: the pushed tasks under `offline`, to see what it does once it can write
   the run's own folders.

Every run keeps its serial log, the agent's output and a screenshot of its report. A run with
no `[BOX]` line, or with a `[BOX-MISSING]` line, is recorded as failed and run again.

## 4. Analysis (days 7–9)

- Each refusal is set beside the task that caused it, and sorted: expected and harmless, a
  sign of overreach, or a sign of compromise.
- We note what the agent was allowed that it did not need, so the policy can shrink.

## 5. Report and read-out (days 9–10)

The [report](report-template.md), the policy files and the evidence, then an hour with your
team.

## What we need from you

A contact who can answer questions about the agents, the scripts and their inputs, and the
recorded model replies or access to record them. Nothing of yours runs outside the NONOS image,
which we boot amnesic: it keeps nothing when it stops.
