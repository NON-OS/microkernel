//! The network step on screen: "No network" first, then the Wi-Fi networks
//! heard, then lines saying what the selected row does.

use crate::network::NETS_MAX;
use crate::render::{self, widgets::lines, widgets::rows};
use crate::state::Context;

use super::network_lines;

const NO_NETWORK: &[u8] = b"No network (default, private)";

pub fn draw(ctx: &Context) {
    let footer: &[u8] = if ctx.net.typing {
        b"ENTER JOIN  ESC CANCEL"
    } else {
        b"ENTER NEXT OR JOIN  S LOOK AGAIN  ESC BACK"
    };
    render::frame(ctx, b"Network", b"No network is the private choice", footer);
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let (buf, x) = (render::buffer(ctx), render::content_x(w));
    let n = &ctx.net;
    let mut items: [&[u8]; 1 + NETS_MAX] = [b""; 1 + NETS_MAX];
    items[0] = NO_NETWORK;
    for (slot, net) in items[1..].iter_mut().zip(n.nets[..n.count].iter()) {
        *slot = net.ssid();
    }
    rows::list(buf, spx, w, h, x, 110, &items[..1 + n.count], n.sel as usize);
    let mut y = 110 + 30 * (1 + n.count as u32) + 12;
    let mut say = |text: &[u8], color: u32| {
        y = lines::text(buf, spx, w, h, x, y, &[text], color);
    };
    network_lines::describe(ctx, &mut say);
}
