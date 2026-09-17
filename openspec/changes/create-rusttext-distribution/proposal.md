## Why

The current RustText prototype proves that a small native GTK editor can be built, but it lacks the mature document, workspace, recovery, search, preview, image, table, math, export, theme, and typography behavior expected from a daily editor. Markion already supplies those capabilities in native Rust under the MIT license. A branded RustText distribution should reuse that proven base instead of reimplementing editor fundamentals.

## What Changes

- Brand the distributable application and native window title as **RustText**, while retaining Markion copyright, license, third-party notices, internal crate names, and upstream attribution.
- Add a built-in **RustText Jade** light theme with a calm green/teal palette and Chinese-friendly Noto font contributions for source, rendered prose, and code.
- Make RustText Jade and Simplified Chinese the first-run/reset defaults. Existing users' explicit persisted theme, font, size, and language preferences remain authoritative.
- Keep all existing theme and typography controls in Preferences, including per-plane font family, font size, paragraph spacing, and custom TOML themes.
- Package Linux output under the RustText product identity without renaming internal Rust crates or workspace member packages.
- Redesign the RustText-owned window interior as **Jade Frost**: translucent jade surfaces, rounded panels, restrained elevation, and compact state effects, while leaving the outer frame to the user's Linux window theme.

Non-goals: removing advanced Markion capabilities; rewriting the editor engine; changing Markdown syntax or document storage; replacing existing user preferences; removing upstream attribution; publishing an upstream GitHub release.

## Capabilities

### New Capabilities

- `rusttext-distribution`: product identity, first-run defaults, attribution, and packaging requirements for the customized distribution.

### Modified Capabilities

- `theme-preferences`: adds RustText Jade to the built-in catalog and makes it the distribution default.
- `ui-i18n`: changes only the distribution's first-run/reset language to Simplified Chinese; unknown persisted language values still follow the existing fallback contract.
- `chrome-platform`: uses RustText as the native product/window title while preserving document-name and dirty-marker behavior.
- `jade-glass`: defines the modern translucent interior, system-decoration boundary, and graceful no-blur fallback.

## Impact

- `src/model.rs`: built-in theme definition and distribution preference defaults.
- `src/app/mod.rs`, title tests, and selected user-facing brand strings: RustText identity.
- `Cargo.toml`, `packager.toml`: product metadata while retaining internal `markion` crate/binary compatibility.
- Documentation: a RustText distribution README with upstream attribution and customization instructions.
- Architecture invariants preserved: no Markdown parsing, document versioning, derived-state caching, highlighting, text-handle reuse, or GPUI-free workspace-member boundaries change.
