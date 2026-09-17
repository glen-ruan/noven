## MODIFIED Requirements

### Requirement: Distribution language default

The RustText distribution SHALL use Simplified Chinese for missing first-run preferences and preference reset. Explicit supported language selections SHALL persist and restore unchanged. Unknown explicit language codes SHALL continue to fall back safely to English through `Language::from_code`.

#### Scenario: First launch is Simplified Chinese

- **WHEN** RustText starts without a preferences file
- **THEN** the active interface language is Simplified Chinese

