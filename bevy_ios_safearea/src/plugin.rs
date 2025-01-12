use bevy::prelude::*;
#[cfg(target_os = "ios")]
use bevy::window::PrimaryWindow;

#[derive(Copy, Clone, Debug, Default)]
pub struct IosSafeArea {
    pub top: f32,
    pub bottom: f32,
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
    window_query: Query<Entity, With<PrimaryWindow>>,
    mut res: ResMut<IosSafeAreaResource>,
) {
    use winit::raw_window_handle::HasWindowHandle;

    info!("safe area updating");

    let entity = window_query.single();
    let raw_window = windows.get_window(entity).expect("invalid window handle");
    if let Ok(handle) = raw_window.window_handle() {
        if let winit::raw_window_handle::RawWindowHandle::UiKit(handle) = handle.as_raw() {
            let ui_view: *mut std::ffi::c_void = handle.ui_view.as_ptr();

            let top = unsafe { crate::native::swift_safearea_top(ui_view) };
            let bottom = unsafe { crate::native::swift_safearea_bottom(ui_view) };

            let safe_area = IosSafeArea { top, bottom };

            info!("safe area updated: {:?}", safe_area);

            res.safe_area = Some(safe_area);
        }
    }
}
