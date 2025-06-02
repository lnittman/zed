# Architecture

Zed organizes its code as a Cargo workspace with many crates.
A similar approach scales well for desktop apps:

1. **Workspace** – define shared dependencies in `Cargo.toml` at the root.
2. **Crates** – split features into crates under `crates/`.
3. **Binary targets** – build launchers like `cli` and `zed`.

The root `Cargo.toml` lists dozens of crates that compose the editor, as seen here:
```toml
# excerpt
members = [
    "crates/activity_indicator",
    "crates/agent",
    ...
]
```

Leveraging a workspace keeps packages independent while enabling cross-crate development.
