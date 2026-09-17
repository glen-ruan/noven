## Identity boundary

`Noven` is the user-facing product name. The Debian package, installed command, root crate, and persisted data paths remain `rusttext` / `markion` compatibility identifiers so an upgrade replaces the existing installation and preserves preferences.

On Linux, the GPUI `app_id` and desktop entry `StartupWMClass` use `io.github.noven.Editor`. The desktop entry continues to execute `rusttext`, but resolves `Icon=noven` from the hicolor icon theme. This makes the application menu and the running Dock window share one visual identity.

## Artwork

The selected ImageGen result is a black rounded square carrying a white folded lowercase `n`. A 512px PNG is the Linux/package master. Smaller hicolor files are resized derivatives of that master. The existing upstream Markion artwork remains available for document examples and upstream attribution; application packaging switches to the Noven asset.

## Compatibility

- Existing `/usr/bin/rusttext` launch habits continue to work.
- Installing the new `.deb` upgrades the existing `rusttext` package rather than creating a duplicate package.
- Existing preferences and session files remain in their current RustText directories.
- An already pinned legacy Dock entry may need to be unpinned and re-pinned because desktop environments cache launcher metadata; the installed launcher itself is updated.

## Unified surfaces

The sidebar uses the same `surface_bg`, content-glass alpha, rounded radius, border, and restrained shadow as the document surface. Tree rows remain transparent at rest, so the new panel reads as one calm plane rather than nested cards.

Code highlighting resolves `CodeTheme::Dark` or `CodeTheme::Light` from the active `ThemeDefinition::is_dark` value at render time. The old persisted field remains in the serialized schema to avoid breaking existing config files, but the Appearance panel no longer exposes or applies it independently.
