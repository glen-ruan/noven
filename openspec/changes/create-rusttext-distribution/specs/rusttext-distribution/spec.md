# rusttext-distribution

## Requirements

### Requirement: RustText distribution identity

The customized distribution SHALL present the product name RustText in its native window title and package metadata while retaining upstream Markion attribution, MIT license text, third-party notices, and internal crate compatibility.

#### Scenario: Native title uses RustText

- **WHEN** a document is active
- **THEN** the native title is `RustText - <filename>` with the existing dirty marker when applicable

#### Scenario: Attribution survives customization

- **WHEN** the source or packaged notices are inspected
- **THEN** the upstream license and third-party notices remain present

### Requirement: Chinese-friendly first-run appearance

The distribution SHALL default new or reset preferences to the RustText Jade theme and Simplified Chinese interface. The theme SHALL contribute installed/common Chinese-capable Noto families independently for source, rendered prose, and code.

#### Scenario: New profile receives distribution defaults

- **WHEN** no preferences file exists
- **THEN** RustText starts in Simplified Chinese with RustText Jade selected

#### Scenario: Existing preferences remain authoritative

- **WHEN** a preferences file explicitly selects another language, theme, or font
- **THEN** that explicit selection is restored instead of being overwritten by distribution defaults

