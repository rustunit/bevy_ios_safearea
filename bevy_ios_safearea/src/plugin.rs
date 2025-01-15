use bevy::prelude::*;

/// Struct providing iOS device safe area insets.
/// It is created and added only when there are insets on the running device.
/// It is recommended to access it from systems by using [`Option<Res<IosSafeArea>>`].
///
/// Using [IosSafeAreaHelper] can simplify accessing it.
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ios_safearea::{IosSafeArea, IosSafeAreaHelper};
///
/// fn bevy_system(safe_area: Option<Res<IosSafeArea>>) {    
///     let safe_area_top = safe_area.top();
/// }
// ```
#[derive(Resource, Clone, Debug, Default)]
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

#[allow(dead_code)]
/// A trait providing helper methods to access the safe area insets.
pub trait IosSafeAreaHelper {
    /// Returns the inset from the top of the screen.
    fn top(&self) -> f32;

    /// Returns the inset from the bottom of the screen.
    fn bottom(&self) -> f32;

    /// Returns the inset from the left side of the screen.
    fn left(&self) -> f32;

    /// Returns the inset from the right side of the screen.
    fn right(&self) -> f32;
}

impl IosSafeAreaHelper for Option<Res<'_, IosSafeArea>> {
    fn top(&self) -> f32 {
        self.as_ref().map_or(0., |a| a.top)
    }

    fn bottom(&self) -> f32 {
        self.as_ref().map_or(0., |a| a.bottom)
    }

    fn left(&self) -> f32 {
        self.as_ref().map_or(0., |a| a.left)
    }

    fn right(&self) -> f32 {
        self.as_ref().map_or(0., |a| a.right)
    }

    pub fn left(&self) -> f32 {
        self.safe_area.map_or(0., |a| a.left)
    }

    pub fn right(&self) -> f32 {
        self.safe_area.map_or(0., |a| a.right)
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
#[allow(dead_code)]
#[derive(Default)]
pub struct IosSafeAreaPlugin;

#[allow(unused_variables)]
impl Plugin for IosSafeAreaPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "ios")]
        app.add_systems(Startup, init);
    }
}

#[cfg(target_os = "ios")]
fn init(
    windows: NonSend<bevy::winit::WinitWindows>,
    window_query: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut commands: Commands,
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

            commands.insert_resource(safe_area);
            tracing::debug!("safe area updated: {:?}", safe_area);
        }
    }
}
