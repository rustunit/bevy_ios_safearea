use bevy::prelude::*;

/// Struct providing iOS device safe area insets.
#[derive(Copy, Clone, Debug, Default)]
pub struct IosSafeArea {
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

#[derive(Resource, Clone, Debug, Default)]
pub struct IosSafeAreaResource {
    pub safe_area: Option<IosSafeArea>,
}

impl IosSafeAreaResource {
    pub fn top(&self) -> f32 {
        self.safe_area.map_or(0., |a| a.top)
    }

    pub fn bottom(&self) -> f32 {
        self.safe_area.map_or(0., |a| a.bottom)
    }

    pub fn left(&self) -> f32 {
        self.safe_area.map_or(0., |a| a.left)
    }

    pub fn right(&self) -> f32 {
        self.safe_area.map_or(0., |a| a.right)
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct IosSafeAreaPlugin;

impl Plugin for IosSafeAreaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IosSafeAreaResource>();
        #[cfg(target_os = "ios")]
        app.add_systems(Startup, init);
    }
}

#[cfg(target_os = "ios")]
fn init(
    windows: NonSend<bevy::winit::WinitWindows>,
    window_query: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut res: ResMut<IosSafeAreaResource>,
) {
    use bevy::utils::tracing;
    use winit::raw_window_handle::HasWindowHandle;

    tracing::debug!("safe area updating");

    let entity = window_query.single();
    let raw_window = windows.get_window(entity).expect("invalid window handle");
    if let Ok(handle) = raw_window.window_handle() {
        if let winit::raw_window_handle::RawWindowHandle::UiKit(handle) = handle.as_raw() {
            let ui_view: *mut std::ffi::c_void = handle.ui_view.as_ptr();

            let top = unsafe { crate::native::swift_safearea_top(ui_view) };
            let bottom = unsafe { crate::native::swift_safearea_bottom(ui_view) };
            let left = unsafe { crate::native::swift_safearea_left(ui_view) };
            let right = unsafe { crate::native::swift_safearea_right(ui_view) };

            let safe_area = IosSafeArea {
                top,
                bottom,
                left,
                right,
            };

            tracing::debug!("safe area updated: {:?}", safe_area);

            res.safe_area = Some(safe_area);
        }
    }
}
