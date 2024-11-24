use gpui::*;
use gpui3 as gpui;

struct WindowContent {
    text: SharedString,
    bounds: Bounds<Pixels>,
    bg: Hsla,
}

fn content_renderer(content: WindowContent) -> impl Fn(&mut gpui::Window, &mut AppContext) -> Div {
    move |window, _cx| {
        let window_bounds = window.bounds();

        div()
            .flex()
            .flex_col()
            .bg(content.bg)
            .size_full()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(content.text.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .text_sm()
                    .items_center()
                    .size_full()
                    .child(format!(
                        "origin: {}, {} size: {}, {}",
                        content.bounds.origin.x,
                        content.bounds.origin.y,
                        content.bounds.size.width,
                        content.bounds.size.height
                    ))
                    .child(format!(
                        "cx.bounds() origin: {}, {} size {}, {}",
                        window_bounds.origin.x,
                        window_bounds.origin.y,
                        window_bounds.size.width,
                        window_bounds.size.height
                    )),
            )
    }
}

fn build_window_options(display_id: DisplayId, bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        // Set the bounds of the window in screen coordinates
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        // Specify the display_id to ensure the window is created on the correct screen
        display_id: Some(display_id),
        titlebar: None,
        window_background: WindowBackgroundAppearance::Transparent,
        focus: false,
        show: true,
        kind: WindowKind::PopUp,
        is_movable: false,
        app_id: None,
        window_min_size: None,
        window_decorations: None,
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Create several new windows, positioned in the top right corner of each screen
        let size = Size {
            width: px(350.),
            height: px(75.),
        };
        let margin_offset = px(150.);

        for screen in cx.displays() {
            let bounds = Bounds {
                origin: point(margin_offset, margin_offset),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Top Left {:?}", screen.id()).into(),
                    bg: gpui::red(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: screen.bounds().upper_right()
                    - point(size.width + margin_offset, -margin_offset),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Top Right {:?}", screen.id()).into(),
                    bg: gpui::red(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: screen.bounds().lower_left()
                    - point(-margin_offset, size.height + margin_offset),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Bottom Left {:?}", screen.id()).into(),
                    bg: gpui::blue(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: screen.bounds().lower_right()
                    - point(size.width + margin_offset, size.height + margin_offset),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Bottom Right {:?}", screen.id()).into(),
                    bg: gpui::blue(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: point(screen.bounds().center().x - size.center().x, margin_offset),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Top Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: point(margin_offset, screen.bounds().center().y - size.center().y),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Left Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: point(
                    screen.bounds().center().x - size.center().x,
                    screen.bounds().center().y - size.center().y,
                ),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: point(
                    screen.bounds().size.width - size.width - margin_offset,
                    screen.bounds().center().y - size.center().y,
                ),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Right Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                }),
            )
            .unwrap();

            let bounds = Bounds {
                origin: point(
                    screen.bounds().center().x - size.center().x,
                    screen.bounds().size.height - size.height - margin_offset,
                ),
                size,
            };

            cx.open_window(
                build_window_options(screen.id(), bounds),
                content_renderer(WindowContent {
                    text: format!("Bottom Center {:?}", screen.id()).into(),
                    bg: gpui::black(),
                    bounds,
                }),
            )
            .unwrap();
        }
    });
}
