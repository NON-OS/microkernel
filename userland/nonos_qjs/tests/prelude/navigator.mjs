// NONOS Operating System (AGPL-3.0-or-later)
// navigator.userAgent has to be the User-Agent the requests carry. A page
// that reads one and a server that logged the other would see two browsers,
// and that mismatch is itself something to fingerprint.

import { readFileSync } from 'node:fs';

export function navigatorChecks(ok, requestSource) {
  const text = readFileSync(requestSource, 'utf8');
  const m = text.match(/pub const USER_AGENT: &str =\s*"([^"]+)";/);
  ok(m !== null, 'the browser names its User-Agent in one constant');
  const ua = m ? m[1] : '';
  ok(globalThis.navigator.userAgent === ua, `navigator says what requests say: ${globalThis.navigator.userAgent}`);
  ok(globalThis.navigator.cookieEnabled === true, 'cookies are on, as they now are');
  ok(globalThis.navigator.platform === 'Win32', 'the platform agrees with the agent string');
}
