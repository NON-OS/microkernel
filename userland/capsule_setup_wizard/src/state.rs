pub struct Context {
    pub base: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub compositor_port: u32,
    pub router_port: u32,
    pub policy_port: u32,
    pub step: u8,
    /// Row on the keyboard screen: an index into the layouts the drivers have.
    pub kbd_sel: u8,
    /// Hours from UTC.
    pub tz_off: i8,
    /// Row on the mode screen: amnesic, USB live, or install.
    pub mode_sel: u8,
    /* The account name typed on the name step. */
    pub name: crate::name::NameState,
    pub wall_sel: u8,
    /// 1 when installed programs may run. Starts at what an earlier boot
    /// decided, so setup shows the standing choice rather than asking again.
    pub local_sel: u8,
    pub local_was: bool,
    /// The disk was still loading when setup asked, so it asks again.
    pub local_pending: bool,
    pub net: crate::network::NetState,
    /* This machine's memory and the Qwen tier chosen by fit to it. */
    pub qwen: crate::qwen::QwenState,
}

impl Context {
    pub fn new(
        base: u64,
        width: u32,
        height: u32,
        stride: u32,
        compositor_port: u32,
        router_port: u32,
        policy_port: u32,
    ) -> Self {
        Self {
            base,
            width,
            height,
            stride,
            compositor_port,
            router_port,
            policy_port,
            step: 0,
            kbd_sel: 0,
            tz_off: 0,
            mode_sel: 0,
            name: crate::name::NameState::new(),
            wall_sel: 0,
            local_sel: 0,
            local_was: false,
            local_pending: false,
            net: crate::network::NetState::new(),
            qwen: crate::qwen::QwenState::read(),
        }
    }
}
