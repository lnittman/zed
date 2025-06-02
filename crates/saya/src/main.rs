use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, prelude::*, px, size};

struct Game;

impl Render for Game {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        pre(
"Saya\n\n1. Explore the forest\n2. Check inventory\n3. Rest at camp"
        )
        .p_4()
        .text_color(gpui::white())
        .bg(gpui::black())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), cx);
        cx.open_window(
            WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() },
            |_, cx| cx.new(|_| Game),
        ).unwrap();
        cx.activate(true);
    });
}
