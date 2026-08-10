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
    #[cfg_attr(
        not(any(target_os = "ios", target_os = "tvos")),
        allow(unused_variables)
    )]
    fn build(&self, app: &mut App) {
        // `init` reacts to `WindowCreated` instead of running once at `Startup`: the
        // winit `UIWindow` is not guaranteed to be registered in `WINIT_WINDOWS` during
        // the first frame — window creation is event-loop-driven on iOS and can complete
        // a frame or two after Bevy's first update. `bevy_winit` writes `WindowCreated`
        // immediately after registering the window, so that message is the earliest
        // point where the handle is guaranteed to be available.
        #[cfg(any(target_os = "ios", target_os = "tvos"))]
        app.add_systems(Update, init);
    }
}

#[cfg(any(target_os = "ios", target_os = "tvos"))]
fn init(
    // Also doubles as the main-thread guarantee `native::safe_area_insets` relies
    // on: Bevy only ever schedules `NonSend` systems on the main thread, and
    // UIKit's `-safeAreaInsets` is main-thread-only.
    _non_send_marker: bevy_ecs::system::NonSendMarker,
    mut window_created: MessageReader<bevy_window::WindowCreated>,
    window: Single<Entity, With<bevy_window::PrimaryWindow>>,
    mut commands: Commands,
) {
    use bevy_log::tracing;
    use winit::raw_window_handle::HasWindowHandle;

    if !window_created
        .read()
        .any(|created| created.window == *window)
    {
        return;
    }

    tracing::debug!("safe area updating");

    let insets = bevy_winit::WINIT_WINDOWS.with_borrow(|windows| {
        // Guaranteed registered at this point (`WindowCreated` is written after
        // registration), but stay panic-free regardless.
        let raw_window = windows.get_window(*window)?;

        let Ok(handle) = raw_window.window_handle() else {
            return None;
        };

        if let winit::raw_window_handle::RawWindowHandle::UiKit(handle) = handle.as_raw() {
            let ui_view: *mut std::ffi::c_void = handle.ui_view.as_ptr();

            // SAFETY: `ui_view` is winit's own live `UIView*` for this window, and
            // this closure runs inside `init`, which only ever runs on the main
            // thread (see the `_non_send_marker` comment above).
            let insets = unsafe { crate::native::safe_area_insets(ui_view) };

            Some(IosSafeAreaResource {
                top: insets.top,
                bottom: insets.bottom,
                left: insets.left,
                right: insets.right,
            })
        } else {
            None
        }
    });

    if let Some(safe_area) = insets {
        tracing::debug!("safe area updated: {:?}", safe_area);
        commands.insert_resource(safe_area);
    }
}
