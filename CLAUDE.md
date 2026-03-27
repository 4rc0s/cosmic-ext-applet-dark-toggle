# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Goal

This codebase is being refactored from a **external monitor brightness + dark mode toggle** applet into a **light/dark mode toggle-only** applet. The DDC/CI external monitor brightness controls are being removed. The goal is a minimal applet that toggles light/dark mode and shows the current state in the status bar icon.

## Build & Development Commands

```sh
just build-debug       # Debug build
just build-release     # Release build (default)
just install           # Install to /usr/share and /usr/bin
just test              # cargo test --workspace --all-features
just fmt               # cargo fmt --all
just fix               # cargo clippy --fix
just pull              # fmt + prettier + fix + test (full pre-PR sweep)
```

Run with logging:
```sh
RUST_LOG="warn,cosmic_ext_applet_dark_toggle=debug" ./target/debug/cosmic-ext-applet-dark-toggle
```

## Architecture

This is a COSMIC desktop applet following the **Elm Model-View-Update** pattern via `libcosmic`.

### Message Flow

All state changes go through `AppMsg` (defined in `app.rs`). The update loop in `Application::update()` pattern-matches on messages and returns `Command`s.

```
User interaction / subscription event
    → AppMsg variant
    → AppState::update() mutates state + returns Command
    → view() re-renders from new state
```

### Key Files

- **`src/app.rs`** — `AppState` struct, `AppMsg` enum, `Application` trait impl. All business logic lives here.
- **`src/view.rs`** — Pure rendering: applet button, popup, monitor controls, dark mode toggle.
- **`src/monitor.rs`** — Async DDC/CI subscription: enumerates `/dev/i2c-*` displays, reads/sets brightness (VCP 0x10). Uses `ddc-hi` crate. **Target for removal.**
- **`src/config.rs`** — `Config` struct persisted via `cosmic_config`. Currently stores per-monitor gamma. **Simplify/remove after monitor removal.**
- **`src/icon.rs`** — `icon_handle!()` macro loading SVGs from `res/icons/`. Currently four brightness-level icons.
- **`src/localize.rs`** — Fluent i18n via `i18n-embed`. Strings in `i18n/*/cosmic_ext_applet_external_monitor_brightness.ftl`.

### Dark Mode Toggle

The current dark mode implementation in `app.rs`:
- Reads `ThemeMode` via `cosmic_config::CosmicConfigEntry`
- Toggles `ThemeMode::is_dark` and writes back with `theme_mode_config.set_is_dark()`
- `AppMsg::ToggleDarkMode` / `AppMsg::DarkMode(bool)` handle the state

### COSMIC Applet Lifecycle

- `AppState::new()` — initializes state, starts config subscription
- `AppState::subscription()` — returns merged subscriptions (monitor DDC, config watcher, theme watcher)
- Popup open/close driven by `AppMsg::TogglePopup` / `AppMsg::CloseRequested`

## Removing Monitor Brightness

Files/sections to remove or gut:
1. `src/monitor.rs` — entire file
2. `src/app.rs` — `monitors` field, `sender` field, all `AppMsg` variants related to brightness/gamma/monitor, the monitor subscription branch
3. `src/view.rs` — `monitor_view()`, brightness slider, gamma controls; simplify `popup_view()` and `applet_button_view()`
4. `src/config.rs` — `MonitorConfig`, `monitors` field in `Config`; keep or remove entirely
5. `src/icon.rs` — replace four brightness icons with two dark/light mode icons
6. `Cargo.toml` — remove `ddc-hi` dependency
7. `i18n/**` — remove `gamma_map` and `refresh` strings; keep `dark_mode`

## i18n

Translation files are at `i18n/<lang>/cosmic_ext_applet_external_monitor_brightness.ftl`. Add new strings in `i18n/en/` first; other languages will fall back to English.

The `fl!("key")` macro (from `src/localize.rs`) resolves strings at runtime.
