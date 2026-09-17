# Noven design QA

## Evidence

- Reference concept: `/home/ruan/.codex/generated_images/01a0aa21-2247-7251-a618-e3ae1ec5a285/exec-c80e0073-61f4-40ec-83f1-e0aaebaaa4d3.png` (1586 x 992).
- Previous implementation screenshot supplied by the user: `/tmp/codex-clipboard-d041894c-b535-42c1-8438-24c4b2c05c97.png` (2460 x 1694).
- Automated final-window capture attempt: `/tmp/rusttext-jade-frost.png` (2360 x 1594), unusable because the Vulkan/X11 surface was captured as black.
- QA viewport is not normalized: the concept uses RustText Jade while the supplied implementation screenshot uses Solarized Dark, and their dimensions differ.

## Comparison history

### Iteration 1

The supplied implementation screenshot showed correct system-owned macOS-style window controls and initial rounded panels, but diverged materially from the concept: the workspace was visually flat, the hierarchy between glass layers was weak, the preview table remained hardcoded white, and expanded preview components did not inherit the active app theme.

Implemented corrections:

- restored server-side window decorations and retained the desktop theme as the sole owner of the titlebar;
- added theme-derived frosted layers, rounded cards, restrained borders and shadows, compact navigation pills, and an editor/preview mode control;
- changed tables, HTML tables, Mermaid states, math/source fallbacks, expanded editors, quotes, links, selections, and visual-table controls to derive their chrome from the active `ThemePalette`;
- kept fenced-code syntax colors independently configurable through the existing code theme;
- added regression tests for system chrome, Jade darkness, frosted theme direction, spacing, and the absence of fixed light component chrome.

### Iteration 2

The user reported that the glass effect still read as too opaque. Inspection found that the previous 0.78 root layer combined with 0.82 content cards into an effective opacity of roughly 96%, visually flattening the desktop background into a solid theme color.

Implemented corrections:

- separated root, chrome, content, component, emphasis, and control glass alpha levels; the default root/content composition is now roughly 81% opaque;
- added persisted Appearance controls for compositor blur, interface opacity (35–95%), ambient particles, and particle density (1–3);
- added a bounded 6/10/14-particle ambient layer behind all interactive content, with slow deterministic motion and no pointer handlers;
- preserved the platform limitation honestly: X11 provides alpha transparency but compositor blur strength remains controlled by the desktop environment;
- reran the full suite: `600 passed, 0 failed, 2 ignored`.

### Iteration 3

The user supplied `/tmp/codex-clipboard-ff777d38-2e80-460c-b710-3c977e0938ea.png` (2460 x 1694). It confirmed the system titlebar and document presentation, but showed a P2 hierarchy mismatch in the left navigation: the sidebar, tab-strip container, resize divider, and every inactive tree/outline row still read as framed filled controls.

Implemented corrections:

- removed the sidebar outer border, shadow, filled background, and visible resize-divider line while preserving its resize hit target;
- removed the Files/Outline tab-strip frame and retained only a translucent active pill and hover feedback;
- made inactive file-tree and outline rows fully transparent, with opacity-scaled active, selected, hover, and drop-target states;
- added a structural regression test preventing framed sidebar chrome from returning;
- reran the complete suite serially: `601 passed, 0 failed, 2 ignored`.

### Iteration 4

The user asked for the Files/Outline region to read as a peer of the document
surface rather than an uncontained strip. The sidebar now uses the same
theme-derived surface color, content opacity, rounded radius, border, and
restrained shadow as the editor while inactive tree rows stay transparent.

The selected minimalist identity was applied as **Noven**: a black rounded
square with a white folded-note `n`. Raster, Windows, macOS, Linux hicolor,
desktop-entry, application-id, window-title, About, and starter-document
assets now share that identity.

### Iteration 5

The user's screenshot
`/tmp/codex-clipboard-551d9f1e-1bd6-478d-b7da-df6d71cb72e2.png`
showed that fenced code still looked like an unrelated fixed navy card next to
the theme-derived display-math surface.

Implemented corrections:

- removed the fixed code-block and copy-button background colors;
- made code containers use the same active-theme surface, opacity, border, and
  shadow pipeline as expanded math and diagram components;
- retained only syntax-token colors, selected automatically from the active
  theme's light/dark direction;
- removed the separate code-theme Light/Dark controls from Appearance;
- restored system-owned Linux window chrome even when the binary is launched
  directly by selecting X11 before GPUI initialization (native Wayland remains
  available through `RUSTTEXT_USE_WAYLAND=1`).

### Iteration 6

The first attempt to resolve the crowded Markdown/Preview control moved it to
a dedicated document toolbar. The user correctly identified that the resulting
mostly-empty row added more visual fragmentation. The final treatment keeps
the switch in the main toolbar but removes its nested card border, fill, and
shadow. A single divider separates it from the menu titles, the switch stays
non-shrinking, and the toolbar itself can scroll horizontally in narrow
windows, preventing label overlap without adding another chrome band.

## Current result

**IMPLEMENTATION VERIFIED; final screenshot comparison remains unavailable.**

The implementation passes the complete root suite (`603 passed, 0 failed, 2
ignored`), refreshed release build, Debian package inspection, and checksum
verification. The refreshed application was launched for interactive
inspection and the system-decoration path was exercised. A pixel-level final
PASS cannot be claimed because desktop capture still records this GPUI/Vulkan
surface as black on the current Linux session.
