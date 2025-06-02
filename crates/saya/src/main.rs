use assets::Assets;
use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, prelude::*, px, size};
use ui::prelude::*;
struct SayaApp;

impl Render for SayaApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = pre(
            "~ Saya ~\n\n1. Explore the forest\n2. Check inventory\n3. Rest at camp",
        )
        .font_family("Zed Mono")
        .p_4()
        .text_color(gpui::white())
        .bg(gpui::black())
        .flex_grow();

        let buttons = ["1", "2", "3", "4"].iter().enumerate().map(|(i, label)| {
            Button::new(format!("option_{i}", i = i + 1), *label)
        });

        v_flex()
            .size_full()
            .font_family("Zed Mono")
            .gap_2()
            .children(vec![
                viewport.into_any_element(),
                h_flex()
                    .gap_2()
                    .children(buttons.map(|b| b.into_any_element()).collect())
                    .into_any_element(),
            ])
    }
}

fn main() {
    Application::new()
        .with_assets(Assets)
        .run(|cx: &mut App| {
            Assets.load_fonts(cx).unwrap();

            let bounds = Bounds::centered(None, size(px(600.0), px(500.0)), cx);
            cx.open_window(
                WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() },
                |_, cx| cx.new(|_| SayaApp),
            ).unwrap();
            cx.activate(true);
        });
}
