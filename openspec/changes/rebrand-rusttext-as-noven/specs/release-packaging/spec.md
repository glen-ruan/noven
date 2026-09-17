## MODIFIED Requirements

### Requirement: Linux package SHALL install application-menu and Dock metadata

The `rusttext` Debian package SHALL install a Noven desktop entry and hicolor Noven icons. The entry SHALL launch `rusttext`, advertise Markdown and plain-text MIME support, and declare the same application/window identity used by the GPUI window so desktop environments can group the launcher and running Dock entry.

#### Scenario: Installed application appears in the application menu

- **WHEN** the user installs or upgrades the Debian package
- **THEN** the desktop environment can list an application named `Noven` in the Office/Text Editor categories
- **AND** launching it executes `rusttext %F`

#### Scenario: Running window uses the Noven Dock icon

- **WHEN** Noven is launched from the installed desktop entry
- **THEN** the running window identity matches the entry's startup class
- **AND** the Dock resolves the installed Noven icon instead of a generic executable icon

#### Scenario: Existing RustText installation upgrades in place

- **WHEN** the new package is installed over an existing `rusttext` package
- **THEN** the package and command names remain `rusttext`
- **AND** no second Debian package is introduced solely for the Noven display name
