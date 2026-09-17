## Why

The installed application still presents a prototype-oriented RustText identity and the upstream Markion icon. The user selected a new restrained black-and-white note mark and needs one coherent identity in the native window, application launcher, desktop shortcut, and running Dock entry.

## What Changes

- Present the product name **Noven** in native window chrome, About surfaces, package metadata, and Linux application menus.
- Add the selected black-and-white folded-`n` note icon as the canonical Noven application artwork.
- Install a freedesktop desktop entry and hicolor application icons so launchers and Docks can resolve the artwork.
- Keep the Debian package and executable command named `rusttext` for clean in-place upgrades from the installed RustText package.
- Give the GPUI window and desktop entry the same Noven application identity so Linux groups the running window with its launcher.
- Give the Files/Outline sidebar its own theme-derived content surface matching the document pane.
- Remove the independent code-theme buttons and derive code highlighting from the active application theme.
- Retain Markion attribution, MIT licensing, existing user data directories, preferences, and internal Rust crate names.

Non-goals: automatically pinning the app to a particular desktop environment's Dock; renaming user configuration directories; changing editor behavior, Markdown storage, or document rendering.

## Capabilities

### Modified Capabilities

- `branding-assets`: adds the selected Noven artwork and makes it the distribution packaging source.
- `release-packaging`: installs a Noven launcher and icon while preserving the `rusttext` package/command upgrade path.
- `chrome-platform`: presents Noven as the window identity without changing filename or dirty-marker behavior.
- `theme-preferences`: keeps code blocks and sidebar surfaces synchronized with the selected application theme.

## Impact

- `assets/`: selected source image and platform icon derivatives.
- `src/app/mod.rs`, `src/i18n.rs`, title tests: visible Noven identity.
- `src/app/root_view.rs`, `src/app/preview.rs`: unified sidebar and code-block theme behavior.
- `packaging/linux/`, `packager.toml`, `Cargo.toml`, `build.rs`: launcher, Dock, and bundle metadata.
- Cached-per-version Markdown state, highlighting, text-handle reuse, and GPUI-free workspace boundaries are untouched.
