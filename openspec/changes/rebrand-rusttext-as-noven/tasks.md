## 1. Brand assets

- [x] 1.1 Add the selected Noven icon to the project and generate packaging derivatives.
- [x] 1.2 Add structural checks for the Noven raster asset and Linux icon installation paths.

## 2. Product identity

- [x] 2.1 Change native title and visible About strings from RustText to Noven while retaining Markion attribution.
- [x] 2.2 Update bundle metadata without renaming the internal crate, Debian package, executable, or user-data directories.

## 3. Linux launcher and Dock integration

- [x] 3.1 Update the desktop entry name, icon, application identity, MIME types, and startup notification metadata.
- [x] 3.2 Install the desktop entry and hicolor icon sizes in the Debian package.
- [x] 3.3 Verify the packaged desktop file resolves to an installed icon and shares identity with the GPUI window.

## 4. Unified application surfaces

- [x] 4.1 Render the sidebar as an independent theme-derived content surface matching the document pane.
- [x] 4.2 Derive code highlighting from the active theme and remove manual code-theme buttons from Appearance.
- [x] 4.3 Add/update structural and behavior tests for the unified surfaces.

## 5. Verification and delivery

- [x] 5.1 Run manual formatting review and targeted brand/title/theme tests (`cargo fmt` is unavailable in the system Rust package).
- [x] 5.2 Run the full root test suite and release build.
- [x] 5.3 Build a refreshed Debian package, portable binary, source archive, and checksums.
- [ ] 5.4 Install the refreshed package and visually verify the title and Dock identity (desktop capture cannot read the GPUI/Vulkan surface).
