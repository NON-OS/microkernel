# IPC

How a [capsule](../overview/glossary-boot-root-record.md#capsule) sends a message through the NONOS kernel: [endpoints](../overview/glossary-dma-pool.md#endpoint) and [inboxes](../overview/glossary-hardware-broker.md#inbox), the size and queue limits, who may send to whom, and how a receiver blocks and times out.

## Parts

- [The model, the calls and limits](ipc-model-calls.md)
- [Who may send to whom](ipc-who-may-send.md)
- [Blocking, timeouts and errors](ipc-blocking-errors.md)
- [Exit, unused code and tests](ipc-exit-tests.md)

## The model

On [the model, the calls and limits page](ipc-model-calls.md#the-model).

## The IPC system calls

On [the model, the calls and limits page](ipc-model-calls.md#the-ipc-system-calls).

## Limits

On [the model, the calls and limits page](ipc-model-calls.md#limits).

## Who may send to whom

On [the who may send to whom page](ipc-who-may-send.md#who-may-send-to-whom).

## Blocking, waking and timeouts

On [the blocking, timeouts and errors page](ipc-blocking-errors.md#blocking-waking-and-timeouts).

## Errors a sender sees

On [the blocking, timeouts and errors page](ipc-blocking-errors.md#errors-a-sender-sees).

## When a capsule exits

On [the exit, unused code and tests page](ipc-exit-tests.md#when-a-capsule-exits).

## Code that is present but not used

On [the exit, unused code and tests page](ipc-exit-tests.md#code-that-is-present-but-not-used).

## Tests and proofs

On [the exit, unused code and tests page](ipc-exit-tests.md#tests-and-proofs).

## See also

- [System calls](syscalls.md): the entry path and dispatch every IPC call goes through.
- [Capabilities](capabilities.md): the token check in front of every call.
- [Processes and capsule spawn](processes-and-spawn.md): where inboxes and endpoints are created and removed.
- [IPC ABI](../abi/ipc.md): the message formats as a capsule author sees them.
- [IPC services](../userland/ipc-services.md): the services capsules talk to.
- [Futex](futex.md): the other blocking primitive.
