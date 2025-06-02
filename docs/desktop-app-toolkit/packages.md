# Packages & Modules

Zed splits functionality into many crates under `crates/`. When creating your own workspace, consider separating concerns similarly:

- `core` – shared types and utilities
- `ui` – views and the gpui-based design system
- `services` – async tasks like networking or database access
- `cli` – command-line interface for launching the app

The `[workspace]` section of `Cargo.toml` enumerates all members. Using features and crate boundaries keeps builds fast and improves modularity.
