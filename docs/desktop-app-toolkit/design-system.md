# Design System

Zed's UI is powered by the [`gpui`](../../crates/gpui) crate. It blends immediate
and retained rendering paradigms.

Key ideas from [`gpui`'s README](../../crates/gpui/README.md):

- State is stored in `Entity` objects managed by GPUI.
- Views implement the `Render` trait and build trees of elements using a
  Tailwind-like API.
- Elements provide low-level control for custom layouts.

These patterns enable fluid, GPU-accelerated interfaces.
