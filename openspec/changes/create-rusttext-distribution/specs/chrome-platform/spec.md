## MODIFIED Requirements

### Requirement: RustText native window identity

The customized distribution SHALL use `RustText` as the native application title prefix while preserving the active filename and unsaved `*` marker.

#### Scenario: Window title identifies document and product

- **WHEN** `notes.md` is active and dirty
- **THEN** the native title is `RustText - notes.md *`

### Requirement: Linux window chrome remains operable

The customized distribution SHALL provide visible close, minimize, and maximize controls and a draggable titlebar when client-side window decorations are used on Linux.

#### Scenario: User manages the window from the custom titlebar

- **WHEN** the RustText main window opens without server-side decorations
- **THEN** a macOS-style traffic-light control group is visible
- **AND** the titlebar can move the window and toggle maximization on double-click
