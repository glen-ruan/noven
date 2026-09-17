## MODIFIED Requirements

### Requirement: Native title uses the Noven product identity

The customized distribution SHALL use `Noven` as the native product/window title while retaining the active filename and existing dirty marker.

#### Scenario: Active clean document

- **WHEN** a clean document is active
- **THEN** the native title is `Noven - <filename>`

#### Scenario: Active dirty document

- **WHEN** the active document has unsaved changes
- **THEN** the native title is `Noven - <filename> *`
