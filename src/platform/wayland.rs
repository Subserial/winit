use crate::{
    event_loop::{EventLoopBuilder, EventLoopWindowTarget},
    monitor::MonitorHandle,
    window::{Window, WindowBuilder},
};

pub use crate::window::Theme;

/// WLR Layer Shell layer type. Maps directly to
/// [`zwlr_layer_shell_v1::layer`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#zwlr_layer_shell_v1:enum:layer).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WLRLayer {
    /// Renders the layer on the background.
    Background,
    /// Renders the layer behind traditional windows.
    #[default]
    Bottom,
    /// Renders the layer above traditional windows.
    Top,
    /// Renders the layer above all other windows.
    Overlay,
}

bitflags::bitflags! {
    /// WLR Layer Surface anchor direction. Maps directly to
    /// [`zwlr_layer_surface_v1::anchor`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#zwlr_layer_surface_v1:enum:anchor).
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct WLRAnchor: u32 {
        /// The top edge of the anchor rectangle.
        const TOP = 1 << 0;
        /// The bottom edge of the anchor rectangle.
        const BOTTOM = 1 << 1;
        /// The left edge of the anchor rectangle.
        const LEFT = 1 << 2;
        /// The right edge of the anchor rectangle.
        const RIGHT = 1 << 3;
    }
}

/// WLR Layer Surface keyboard interactivity. Maps directly to
/// [`zwlr_layer_surface_v1::keyboard_interactivity`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#zwlr_layer_surface_v1:enum:keyboard_interactivity).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WLRKeyboardInteractivity {
    /// The surface is not interested in keyboard events.
    #[default]
    None,
    /// The surface wants exclusive access to keyboard events.
    /// If the surface layer is Background or Bottom, this is equivalent to OnDemand.
    Exclusive,
    /// The surface wants to receive keyboard events when focused.
    OnDemand,
}

/// WLR Exclusive zone. Maps to input to
/// [`zwlr_layer_surface_v1::set_exclusive_zone`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#zwlr_layer_surface_v1:request:set_exclusive_zone).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WLRExclusiveZone {
    /// The surface has no exclusive zone and may be moved by the compositor, like a notification.
    #[default]
    None,
    /// The surface should ignore other exclusive zones, like a wallpaper or lock screen.
    IgnoreOthers,
    /// The surface should have an exclusive zone, specified in pixels.
    Positive(i32),
}

/// Additional methods on [`EventLoopWindowTarget`] that are specific to Wayland.
pub trait EventLoopWindowTargetExtWayland {
    /// True if the [`EventLoopWindowTarget`] uses Wayland.
    fn is_wayland(&self) -> bool;
}

impl<T> EventLoopWindowTargetExtWayland for EventLoopWindowTarget<T> {
    #[inline]
    fn is_wayland(&self) -> bool {
        self.p.is_wayland()
    }
}

/// Additional methods on [`EventLoopBuilder`] that are specific to Wayland.
pub trait EventLoopBuilderExtWayland {
    /// Force using Wayland.
    fn with_wayland(&mut self) -> &mut Self;

    /// Whether to allow the event loop to be created off of the main thread.
    ///
    /// By default, the window is only allowed to be created on the main
    /// thread, to make platform compatibility easier.
    fn with_any_thread(&mut self, any_thread: bool) -> &mut Self;
}

impl<T> EventLoopBuilderExtWayland for EventLoopBuilder<T> {
    #[inline]
    fn with_wayland(&mut self) -> &mut Self {
        self.platform_specific.forced_backend = Some(crate::platform_impl::Backend::Wayland);
        self
    }

    #[inline]
    fn with_any_thread(&mut self, any_thread: bool) -> &mut Self {
        self.platform_specific.any_thread = any_thread;
        self
    }
}

/// Additional methods on [`Window`] that are specific to Wayland.
pub trait WindowExtWayland {}

impl WindowExtWayland for Window {}

/// Additional methods on [`WindowBuilder`] that are specific to Wayland.
pub trait WindowBuilderExtWayland {
    /// Build window with the given name.
    ///
    /// The `general` name sets an application ID, which should match the `.desktop`
    /// file destributed with your program. The `instance` is a `no-op`.
    ///
    /// For details about application ID conventions, see the
    /// [Desktop Entry Spec](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#desktop-file-id)
    fn with_name(self, general: impl Into<String>, instance: impl Into<String>) -> Self;
    /// Create this window using the WLR Layer Shell protocol.
    ///
    /// Building this window will fail if the compositor does not support the `zwlr_layer_shell_v1`
    /// protocol.
    fn with_layer_shell(self, layer: WLRLayer) -> Self;

    fn with_anchor(self, anchor: WLRAnchor) -> Self;

    fn with_exclusive_zone(self, exclusive_zone: WLRExclusiveZone) -> Self;

    fn with_margin(self, top: i32, right: i32, bottom: i32, left: i32) -> Self;

    fn with_keyboard_interactivity(self, keyboard_interactivity: WLRKeyboardInteractivity) -> Self;
}

impl WindowBuilderExtWayland for WindowBuilder {
    #[inline]
    fn with_name(mut self, general: impl Into<String>, instance: impl Into<String>) -> Self {
        self.platform_specific.name = Some(crate::platform_impl::ApplicationName::new(
            general.into(),
            instance.into(),
        ));
        self
    }

    #[inline]
    fn with_layer_shell(mut self, layer: WLRLayer) -> Self {
        self.platform_specific.wayland.layer_shell = Some(layer);
        self
    }

    #[inline]
    fn with_anchor(mut self, anchor: WLRAnchor) -> Self {
        self.platform_specific.wayland.anchor = Some(anchor);
        self
    }

    #[inline]
    fn with_exclusive_zone(mut self, exclusive_zone: WLRExclusiveZone) -> Self {
        self.platform_specific.wayland.exclusive_zone = Some(exclusive_zone);
        self
    }

    #[inline]
    fn with_margin(mut self, top: i32, right: i32, bottom: i32, left: i32) -> Self {
        self.platform_specific.wayland.margin = Some((top, right, bottom, left));
        self
    }

    #[inline]
    fn with_keyboard_interactivity(
        mut self,
        keyboard_interactivity: WLRKeyboardInteractivity,
    ) -> Self {
        self.platform_specific.wayland.keyboard_interactivity = Some(keyboard_interactivity);
        self
    }
}

/// Additional methods on `MonitorHandle` that are specific to Wayland.
pub trait MonitorHandleExtWayland {
    /// Returns the inner identifier of the monitor.
    fn native_id(&self) -> u32;
}

impl MonitorHandleExtWayland for MonitorHandle {
    #[inline]
    fn native_id(&self) -> u32 {
        self.inner.native_identifier()
    }
}
