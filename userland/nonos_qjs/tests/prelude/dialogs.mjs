// NONOS Operating System (AGPL-3.0-or-later)
// alert, confirm and prompt were not defined, so the call threw. A script
// cannot wait for the reader here, so each hands the browser what it asked
// and answers at once, and never with a yes.

export function dialogChecks(ok) {
  const asked = [];
  globalThis.__njs_dialog = (k, m) => asked.push(`${k}:${m}`);

  ok(globalThis.alert('Saved') === undefined, 'alert answers nothing');
  ok(globalThis.confirm('Delete it?') === false, 'confirm answers no');
  ok(globalThis.prompt('Your name?', 'Ann') === null, 'prompt answers none, not its default');
  globalThis.alert();
  globalThis.alert(42);
  ok(asked.join('|') === '0:Saved|1:Delete it?|2:Your name?|0:|0:42',
    `each reaches the browser as text: ${asked.join('|')}`);
}
