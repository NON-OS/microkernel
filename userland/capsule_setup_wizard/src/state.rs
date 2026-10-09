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
    /* The boot menu asked to install, so the mode screen starts on Install. */
    pub install_boot: bool,
    /* The account name typed on the name step. */
    pub name: crate::name::NameState,
    pub host: crate::host::HostState,
    /* The highlighted row of the Appearance step. */
    pub wall_sel: u8,
    /* The desktop's wallpaper, by catalog index; always one of those kept. */
    pub wall_desktop: u8,
    /* The wallpapers kept, as nonos_policy_proto::wallpapers_kept. */
    pub walls_kept: u64,
    /// 1 when installed programs may run. Starts at what an earlier boot
    /// decided, so setup shows the standing choice rather than asking again.
    pub local_sel: u8,
    pub local_was: bool,
    /// The disk was still loading when setup asked, so it asks again.
    pub local_pending: bool,
    pub net: crate::network::NetState,
    /* Row on the route screen: the default network, the mixnet first. */
    pub route_sel: u8,
    /* This machine's memory and the Qwen tier chosen by fit to it. */
    pub qwen: crate::qwen::QwenState,
    /* The app switches this image carries, those turned off, and the row. */
    pub apps_present: u8,
    pub apps_off: u8,
    pub apps_sel: u8,
    /* The answers the settings service refused when setup applied them, as
     * render::screens::unsaved bits, and whether the review has said so. */
    pub unsaved: u16,
    pub unsaved_told: bool,
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
        let install_boot = crate::setup::machine::install_boot();
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
            mode_sel: if install_boot { crate::render::screens::mode::INSTALL } else { 0 },
            install_boot,
            name: crate::name::NameState::new(),
            host: crate::host::HostState::new(),
            wall_sel: crate::render::screens::appearance::DESKTOP_DEFAULT,
            wall_desktop: crate::render::screens::appearance::DESKTOP_DEFAULT,
            walls_kept: nonos_policy_proto::wallpapers_kept::ALL,
            local_sel: 0,
            local_was: false,
            local_pending: false,
            net: crate::network::NetState::new(),
            route_sel: 0,
            qwen: crate::qwen::QwenState::read(),
            apps_present: crate::apps::present(),
            apps_off: 0,
            apps_sel: 0,
            unsaved: 0,
            unsaved_told: false,
        }
    }
}
