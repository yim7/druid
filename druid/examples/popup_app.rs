#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::{
    widget::TextBox, AppLauncher, Data, Lens, Point, Size, Widget, WidgetExt, WindowDesc,
    WindowHandle, WindowLevel,
};

#[derive(Clone, Default, Data, Lens)]
struct AppState {
    input: String,
}

fn window_builder() -> WindowDesc<AppState> {
    let size = Size::new(500.0, 50.0);
    let pos = Point::new(660.0, 340.0);

    WindowDesc::new(app_ui())
        .window_size(size)
        .set_level(WindowLevel::Tooltip(WindowHandle::default()))
        .set_position(pos)
}

fn app_ui() -> impl Widget<AppState> {
    TextBox::new().padding(10.0).lens(AppState::input)
}

fn main() {
    AppLauncher::with_window(window_builder())
        .log_to_console()
        .launch(AppState::default())
        .expect("launch failed");
}
