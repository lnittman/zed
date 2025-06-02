# Design System

Zed's UI is powered by the [`gpui`](../../crates/gpui) crate. It blends immediate
and retained rendering paradigms.

Key ideas from [`gpui`'s README](../../crates/gpui/README.md):

- State is stored in `Entity` objects managed by GPUI.
- Views implement the `Render` trait and build trees of elements using a
  Tailwind-like API.
- Elements provide low-level control for custom layouts.

These patterns enable fluid, GPU-accelerated interfaces.

## Custom ASCII Style

`gpui` is flexible enough to drive other design languages. The `saya` example
registers the bundled fonts using `assets::Assets` and then applies a monospace
font to the entire view:

```rust
use assets::Assets;
use ui::prelude::*;

Application::new()
    .with_assets(Assets)
    .run(|cx| {
        Assets.load_fonts(cx).unwrap();
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| SayaApp)
        }).unwrap();
    });
```

Inside the `Render` implementation the root container sets the font family to
`Zed Mono` and arranges the viewport and buttons using `v_flex` and `h_flex`
helpers. This produces an old terminal look and serves as a starting point for
building a full ASCII design system.
