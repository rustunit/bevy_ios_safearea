use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, system::SystemParam};

/// Resource providing iOS device safe area insets.
/// It is created and added only when there are insets on the running device.
/// It is recommended to access it from systems by using [`IosSafeArea`] SystemParam.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ios_safearea::IosSafeArea;
///
/// fn bevy_system(safe_area: IosSafeArea) {
///     let safe_area_top = safe_area.top();
/// }
// ```
#[derive(Resource, Clone, Debug, Default)]
pub struct IosSafeAreaResource {
    /// The inset from the top of the screen.
    ///
    /// This value accounts for elements like the notch or status bar.
    pub top: f32,
    /// The inset from the bottom of the screen.
    ///
    /// This value accounts for elements like the home indicator.
    pub bottom: f32,
    /// The inset from the left side of the screen.
    ///
    /// This value is non-zero for devices with rounded corners or unique screen shapes.
    pub left: f32,
    /// The inset from the right side of the screen.
    ///
    /// This value is non-zero for devices with rounded corners or unique screen shapes.
    pub right: f32,
}

/// SystemParam helper allowing to read insets while defaulting to 0 if not available.
#[derive(SystemParam)]
pub struct IosSafeArea<'w> {
    resource: Option<Res<'w, IosSafeAreaResource>>,
}

impl IosSafeArea<'_> {
    /// top inset
    pub fn top(&self) -> f32 {
        self.resource.as_ref().map(|r| r.top).unwrap_or(0.)
    }

    /// bottom inset
    pub fn bottom(&self) -> f32 {
        self.resource.as_ref().map(|r| r.bottom).unwrap_or(0.)
    }

    /// left inset
    pub fn left(&self) -> f32 {
        self.resource.as_ref().map(|r| r.left).unwrap_or(0.)
    }

    /// right inset
    pub fn right(&self) -> f32 {
        self.resource.as_ref().map(|r| r.right).unwrap_or(0.)
    }
}

/// Plugin to query iOS device safe area insets.
///
/// # Example
/// ```no_run
/// use bevy::prelude::*;
///
/// App::new()
///     .add_plugins((DefaultPlugins,bevy_ios_safearea::IosSafeAreaPlugin))
///     .run();
/// ```
#[derive(Default)]
pub struct IosSafeAreaPlugin;

impl Plugin for IosSafeAreaPlugin {
    #[cfg_attr(not(target_os = "ios"), allow(unused_variables))]
    fn build(&self, app: &mut App) {
        // `init` runs in `Update` (not `Startup`) because the winit `UIWindow` is not
        // guaranteed to be registered in `WINIT_WINDOWS` during the first frame — window
        // creation is event-loop-driven on iOS and can complete a frame or two after
        // Bevy's first update. `init` retries until the handle is available, then
        // disables itself.
        #[cfg(target_os = "ios")]
        app.add_systems(Update, init);
    }
}

#[cfg(target_os = "ios")]
fn init(
    _non_send_marker: bevy_ecs::system::NonSendMarker,
    window: Single<Entity, With<bevy_window::PrimaryWindow>>,
    mut commands: Commands,
    mut done: Local<bool>,
) {
    use bevy_log::tracing;
    use winit::raw_window_handle::HasWindowHandle;

    if *done {
        return;
    }

    tracing::debug!("safe area updating");

    let insets = bevy_winit::WINIT_WINDOWS.with_borrow(|windows| {
        // The OS window may not be registered yet during early frames; retry next
        // frame instead of panicking.
        let Some(raw_window) = windows.get_window(*window) else {
            tracing::debug!("safe area: window handle not ready yet, retrying next frame");
            return None;
        };

        let Ok(handle) = raw_window.window_handle() else {
            return None;
        };

        if let winit::raw_window_handle::RawWindowHandle::UiKit(handle) = handle.as_raw() {
            let ui_view: *mut std::ffi::c_void = handle.ui_view.as_ptr();

            let (top, bottom, left, right) = unsafe {
                (
                    crate::native::swift_safearea_top(ui_view),
                    crate::native::swift_safearea_bottom(ui_view),
                    crate::native::swift_safearea_left(ui_view),
                    crate::native::swift_safearea_right(ui_view),
                )
            };

            Some(IosSafeAreaResource {
                top,
                bottom,
                left,
                right,
            })
        } else {
            None
        }
    });

    if let Some(safe_area) = insets {
        tracing::debug!("safe area updated: {:?}", safe_area);
        commands.insert_resource(safe_area);
        *done = true;
    }
}
