## 1. Distribution defaults

- [x] 1.1 Add the RustText Jade built-in theme with Noto font contributions and tests.
- [x] 1.2 Set first-run/reset defaults to RustText Jade and Simplified Chinese, preserving explicit persisted preferences.

## 2. Product identity and packaging

- [x] 2.1 Change native window/product identity to RustText and update affected tests.
- [x] 2.2 Update Linux packaging metadata while retaining the internal `markion` crate and binary name.
- [x] 2.3 Add RustText distribution documentation and preserve upstream MIT attribution.

## 3. Verification

- [x] 3.1 Run formatting and targeted preference/theme/title tests.
- [x] 3.2 Run the full root test suite and release build.
- [x] 3.3 Produce a directly runnable Linux binary and source archive.

## 4. Linux window chrome

- [x] 4.1 Add macOS-style close, minimize, and maximize controls plus a draggable titlebar.
- [x] 4.2 Add structural tests and verify the corrected Linux release build.
- [x] 4.3 Rebuild the Debian package and increment its package revision.

## 5. Jade Frost redesign

- [x] 5.1 Return outer chrome to the system theme and add a packaged X11 launcher fallback.
- [x] 5.2 Implement the dark-jade translucent palette, rounded workspace surfaces, navigation pills, restrained interaction states, and theme-derived preview/table/expanded-block chrome.
- [ ] 5.3 Add structural/theme tests and complete visual QA against the selected concept.
- [x] 5.4 Build and package the `0.3.10-3` Linux revision.

## 6. Adjustable visual effects

- [x] 6.1 Add persisted blur, opacity, particle, and particle-density preferences.
- [x] 6.2 Add live Appearance controls beside the code-block controls.
- [x] 6.3 Add a bounded non-interactive ambient particle layer and opacity scaling.
- [x] 6.4 Run the full regression suite.
- [x] 6.5 Build and package the refreshed `0.3.10-4` Linux revision.
- [x] 6.6 Remove framed sidebar/tree chrome and retain only translucent interaction states.
