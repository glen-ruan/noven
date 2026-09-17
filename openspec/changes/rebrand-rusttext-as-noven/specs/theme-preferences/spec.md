## MODIFIED Requirements

### Requirement: Theme selection governs document and navigation surfaces

The selected application theme SHALL govern the document surface, Files/Outline sidebar surface, expanded preview components, and code highlighting. Code highlighting SHALL automatically select a compatible light or dark palette from the active theme and SHALL NOT expose a separate Light/Dark selector in Appearance.

#### Scenario: Switching to a dark theme

- **WHEN** the user selects a dark application theme
- **THEN** the editor, sidebar, expanded preview components, and code blocks render with their dark theme-compatible surfaces and palette

#### Scenario: Switching to a light theme

- **WHEN** the user selects a light application theme
- **THEN** the editor, sidebar, expanded preview components, and code blocks render with their light theme-compatible surfaces and palette

#### Scenario: Appearance has no independent code-theme selector

- **WHEN** the user opens Appearance preferences
- **THEN** no separate Light/Dark code-highlight theme buttons are shown
- **AND** existing persisted `code_theme` values remain safe to read for backward compatibility but do not override the active application theme
