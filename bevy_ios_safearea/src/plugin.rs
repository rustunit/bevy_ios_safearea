use bevy::{ecs::system::SystemParam, prelude::*};

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
        #[cfg(target_os = "ios")]
        app.add_systems(Startup, init);
    }
}

#[cfg(target_os = "ios")]
fn init(
    _non_send_marker: bevy::ecs::system::NonSendMarker,
    window: Single<Entity, With<bevy::window::PrimaryWindow>>,
    mut commands: Commands,
) {
    use bevy::log::tracing;
    use winit::raw_window_handle::HasWindowHandle;

    use bevy::winit::WINIT_WINDOWS;

    tracing::debug!("safe area updating");
    WINIT_WINDOWS.with_borrow_mut(|windows| {
        let raw_window = windows.get_window(*window).expect("invalid window handle");
        if let Ok(handle) = raw_window.window_handle() {
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

                let safe_area = IosSafeAreaResource {
                    top,
                    bottom,
                    left,
                    right,
                };

                tracing::debug!("safe area updated: {:?}", safe_area);

                commands.insert_resource(safe_area);
            }
        }
    });
}
