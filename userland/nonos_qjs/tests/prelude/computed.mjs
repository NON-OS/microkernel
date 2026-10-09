// NONOS Operating System (AGPL-3.0-or-later)
// getComputedStyle answered an empty string for everything. It now asks the
// host for each property it is read for, in either spelling, and answers
// empty for something that is not an element.

export function computedChecks(ok) {
  const asked = [];
  globalThis.__njs_computed = (n, p) => { asked.push(`${n}:${p}`); return p === 'display' ? 'none' : ''; };
  const el = { __node: 7 };
  const cs = globalThis.getComputedStyle(el);
  ok(cs.display === 'none', 'a property reads through the host');
  ok(cs.getPropertyValue('margin-top') === '', 'getPropertyValue too');
  ok(cs.backgroundColor === '', 'and the script spelling');
  ok(asked.join() === '7:display,7:margin-top,7:backgroundColor', `the host is asked: ${asked.join()}`);
  ok(globalThis.getComputedStyle(null).display === '', 'no element, nothing');
  ok(asked.length === 3, 'and the host is not asked for it');
  ok(cs.getPropertyPriority('display') === '', 'no priority is claimed');
}
