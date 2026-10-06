// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/* Display roles, flow margins, headings and lists. [hidden] is an
 * ordinary UA rule, so an author display wins over it as in Chromium. */
pub(super) const SHEET: &str = concat!(
    "[hidden],area,base,basefont,datalist,head,link,meta,noembed,noframes,param,rp,",
    "script,style,template,title{display:none}",
    "dialog:not([open]){display:none}[popover]{display:none}",
    "html,address,blockquote,body,center,dialog,div,figure,figcaption,footer,form,",
    "header,hr,legend,listing,main,p,plaintext,pre,search,xmp,article,aside,",
    "h1,h2,h3,h4,h5,h6,hgroup,nav,section,dir,dd,dl,dt,menu,ol,ul,details,summary,",
    "fieldset,optgroup,frameset,frame{display:block}",
    "li{display:list-item}",
    "body{margin:8px}",
    "p,dl,ol,ul,menu,dir,pre,listing,xmp,plaintext{margin-top:1em;margin-bottom:1em}",
    "blockquote,figure{margin:1em 40px}",
    "dd{margin-left:40px}",
    "ol,ul,menu,dir{padding-left:40px}",
    "ol ol,ol ul,ol menu,ul ol,ul ul,ul menu,menu ol,menu ul,menu menu",
    "{margin-top:0;margin-bottom:0}",
    "h1{font-size:2em;margin-top:.67em;margin-bottom:.67em}",
    "h2{font-size:1.5em;margin-top:.83em;margin-bottom:.83em}",
    "h3{font-size:1.17em;margin-top:1em;margin-bottom:1em}",
    "h4{margin-top:1.33em;margin-bottom:1.33em}",
    "h5{font-size:.83em;margin-top:1.67em;margin-bottom:1.67em}",
    "h6{font-size:.67em;margin-top:2.33em;margin-bottom:2.33em}",
    "h1,h2,h3,h4,h5,h6,th,b,strong{font-weight:bold}",
    "hr{margin:.5em auto;border:1px inset #808080;color:#808080}",
    "center{text-align:-webkit-center}",
    "fieldset{margin:0 2px;padding:.35em .75em .625em;border:2px groove #c0c0c0}",
    "legend{padding-left:2px;padding-right:2px}",
    "iframe{border:2px inset #808080}",
);
