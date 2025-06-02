# Building & Packaging

Zed provides scripts for each platform in `docs/src/development`.
The basic workflow on macOS is:

```bash
cargo run
```

Release builds use `cargo run --release`.
For backend services (collaboration, AI), dependencies can be started with:

```bash
docker compose up -d
```

Similar instructions exist for Linux and Windows. Consult the [`macos.md`](../../docs/src/development/macos.md) and other platform docs for troubleshooting tips.

On macOS you can generate a signed application bundle using:

```bash
./script/bundle-mac <AppName>
```

The resulting `.app` will be placed in `target/release/` by default.
