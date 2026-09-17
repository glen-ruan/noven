## MODIFIED Requirements

### Requirement: Distribution application artwork

The customized distribution SHALL provide the selected Noven application artwork as a reusable raster master and SHALL use derivatives of that artwork for application packaging and Linux launcher/Dock integration. Upstream Markion artwork MAY remain available for document examples and attribution.

#### Scenario: Linux launcher resolves Noven artwork

- **WHEN** the Debian package is installed
- **THEN** the desktop entry's `Icon=noven` resolves to an installed hicolor application icon derived from the selected Noven artwork

#### Scenario: Small Dock icon stays identifiable

- **WHEN** a desktop environment renders the Noven icon at 32px or 48px
- **THEN** the black rounded-square background and white folded-`n` note mark remain visually distinct
