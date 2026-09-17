## ADDED Requirements

### Requirement: Jade Frost application interior

RustText SHALL provide a calm dark-jade visual system with translucent application surfaces, rounded panels, restrained shadows, hairline borders, and readable long-form typography.

#### Scenario: Glass effects remain usable without compositor blur

- **WHEN** the Linux compositor supports background blur
- **THEN** RustText requests a blurred window background
- **AND WHEN** compositor blur is unavailable
- **THEN** sufficiently opaque theme surfaces preserve contrast and hierarchy

### Requirement: System-owned outer window frame

RustText SHALL request server-side window decorations and SHALL NOT render a duplicate in-app titlebar or traffic-light controls.

#### Scenario: User window theme owns window chrome

- **WHEN** RustText launches through the packaged Linux command
- **THEN** the available X11 window manager renders the titlebar, borders, and window buttons
- **AND** the RustText interior begins with its own menu and workspace chrome below that frame

### Requirement: Low-cost interaction effects

RustText SHALL use hover, active, focus, opacity, and elevation states that do not add document parsing or per-keystroke derived-state work.

#### Scenario: Typing with Jade Frost enabled

- **WHEN** the user types in source or visual-edit mode
- **THEN** visual effects reuse the existing theme palette and GPUI element states
- **AND** no additional Markdown parse or document mutation occurs

### Requirement: User-adjustable visual effects

RustText SHALL expose persisted visual-effect controls beside the code-block appearance controls. The controls SHALL include compositor blur, interface opacity, ambient particles, and particle density.

#### Scenario: Adjusting glass and particles

- **WHEN** the user changes a visual-effect control in Preferences → Appearance
- **THEN** the active window repaints immediately without changing document content
- **AND** the selected values survive application restart
- **AND** disabling compositor blur retains the selected plain transparency
- **AND** ambient particles remain behind content and do not accept pointer input

### Requirement: Theme-derived document component chrome

RustText SHALL derive tables, HTML tables, diagram containers, math fallbacks, source islands, expanded block editors, links, selections, quote chrome, and table controls from the active application theme. Fenced code blocks SHALL retain their separately configurable code theme.

#### Scenario: Switching between light and dark application themes

- **WHEN** the user selects another built-in or custom application theme
- **THEN** all non-code document components repaint from that theme's surface, panel, border, text, muted, and active colors
- **AND** no fixed light table or expanded-panel background remains visible in a dark theme
- **AND** fenced code blocks continue using the independently selected code theme

### Requirement: Borderless sidebar navigation

RustText SHALL render the Files and Outline sidebar as lightweight navigation over the workspace glass rather than as a framed, filled panel.

#### Scenario: Browsing files and headings

- **WHEN** the sidebar is visible
- **THEN** its outer container, tab-strip container, and resize divider have no visible border or opaque fill
- **AND** inactive tree and outline rows remain transparent
- **AND** active, hover, selected, and drop-target rows retain theme-derived translucent feedback
