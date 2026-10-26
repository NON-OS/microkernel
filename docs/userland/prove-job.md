# The prover job

`capsule_prove` runs headless as a job: init spawns it for one asker and one op, PROVE or DELIVER, and it serves that one job, then exits. Exit wipes its memory. The wire is `nonos_proof_abi::wire` (`userland/nonos_proof_abi/src/wire/prove.rs`).

## One job

- The first frame must be the op and pid init bound it to; any other first frame is answered and starts nothing (`userland/capsule_prove/src/job/serve.rs`, `fn serve`).
- PROVE or DELIVER is answered at once with the job id, then run stage by stage (`userland/capsule_prove/src/job/start.rs`, `fn start`).
- Between stages the job answers what has queued up and does not wait for more, so a stage never waits on its asker (`userland/capsule_prove/src/job/drain.rs`, `fn drain`).
- STATUS and GET answer only the pid whose PROVE made the job. Anyone else gets ENOENT (-2), and a second PROVE or DELIVER gets EBUSY (-16) (`userland/capsule_prove/src/job/answer.rs`, `fn answer`).
- Every frame is taken with `MkIpcRecvReq` and every answer but the first goes by `MkIpcReplyTo` with that frame's request id, so the job learns whether the call took it: 0 says it did; ETIMEDOUT says the call gave up first and the answer reached no one (`userland/capsule_prove/src/job/serve_frame.rs`, `fn frame`; `drain.rs`).

## When it exits

`exit_rule.rs` decides (`userland/capsule_prove/src/job/exit_rule.rs`, `fn exits`):

| The job | It exits |
|---|---|
| Done | once GET has handed out the container's last byte |
| Failed | once the asker's call took a STATUS answer carrying the failure code (`MkIpcReplyTo` returned 0) |
| Failed, and the asker sends nothing | 30 s (`FAILED_QUIET_MS`) after the failure or the asker's last frame |
| any job, or none yet | after 10 minutes (`QUIET_MS`) with no request at all |

So a failure is read once and then frees the prover for the next ask. Until a failed job exits, a new ask is still EBUSY: at most 30 s, not the 10 minutes it was before.

STATUS from anyone but the asker does not count as reading the failure. Nor does an answer that reached no one: the asker's next STATUS carries the failure again (`userland/capsule_prove/src/job/state_ops.rs`, `fn replied`).

The serve loop's receive waits no longer than the nearer of these deadlines, and at least 1 ms, since a wait of 0 is for ever (`fn wait_ms`).

## Proof

`userland/prove_proofs` mounts `answer.rs`, `state.rs`, `state_ops.rs` and `exit_rule.rs` as they ship (`src/job/mod.rs`):

- `tests/job_exit_tests.rs`: a failed job tells its asker the code, then exits; another pid's STATUS frees nothing; an unread failure exits after 30 s, and a frame from the asker restarts the 30 s; a Done job exits once fetched, never at 30 s.
- `tests/job_told_tests.rs`: a failure is read only when its answer was taken (rc 0); after ETIMEDOUT or ENOENT nothing is read, a later frame reads nothing, and the next STATUS carries it again; an answer that did not carry the failure reads nothing.
- `tests/job_rc_tests.rs`: both places the job answers from send by request id and pass the return on.
- Host mutants: `verification/host_mutants/prove-told-rc.json`.
- `tests/job_wait_tests.rs`: the quiet spell; the wait reaches the nearer deadline and is never 0; neither receive in the job waits for ever.
