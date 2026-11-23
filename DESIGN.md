---
name: PDFree
description: An offline Windows PDF viewer and editor that works like a local workbench.
colors:
  window-background: "#f5f7fa"
  window-background-dark: "#0b1220"
  surface: "#ffffff"
  surface-dark: "#16202e"
  surface-alt: "#f4f6f9"
  surface-alt-dark: "#111a26"
  canvas: "#e6ebf1"
  canvas-dark: "#0e1622"
  border: "#dbe3ec"
  border-dark: "#2b3a4d"
  text-primary: "#223b54"
  text-primary-dark: "#e6edf5"
  text-secondary: "#526171"
  text-secondary-dark: "#9db0c3"
  accent: "#146bca"
  accent-dark: "#5aa2e8"
  warning-background: "#fff8e8"
  warning-background-dark: "#3a2f14"
  warning: "#805715"
  warning-dark: "#f0c36d"
  page: "#ffffff"
  scrim: "#172b4d70"
  selection: "#3a8de65c"
  highlight-yellow: "#ffda326b"
  note: "#ffe475"
  review: "#d68c00"
typography:
  wordmark:
    fontSize: "19px"
    fontWeight: 700
  title:
    fontSize: "20px"
    fontWeight: 700
  dialog-title:
    fontSize: "21px"
    fontWeight: 600
  display:
    fontSize: "25px"
    fontWeight: 600
  body:
    fontSize: "14px"
    fontWeight: 400
  label:
    fontSize: "12px"
    fontWeight: 400
rounded:
  sm: "4px"
  md: "8px"
spacing:
  xs: "4px"
  s: "8px"
  m: "12px"
  l: "16px"
  modal: "20px"
components:
  toolbar-bar:
    backgroundColor: "{colors.surface}"
    height: "44px"
  status-bar:
    backgroundColor: "{colors.surface}"
    height: "32px"
  floating-search:
    backgroundColor: "{colors.surface}"
    rounded: "{rounded.md}"
    padding: "8px"
    height: "44px"
  modal-dialog:
    backgroundColor: "{colors.surface}"
    rounded: "{rounded.md}"
    padding: "20px"
  viewport:
    backgroundColor: "{colors.canvas}"
  page-sheet:
    backgroundColor: "{colors.page}"
---

# Design System: PDFree

## Overview

**Creative North Star: "The Local Workbench"**

PDFree is a tool that lives on the desk, not in a browser tab. Its chrome is a set of close-to-hand controls arranged in fixed bars, the way a workbench keeps its tools in reach. Everything it needs is local, so the interface never implies that a file is leaving the machine. The page is the work, and the controls are arranged around it for fast, repeated use.

The system is dense and precise. Two toolbar rows sit above the document, a status bar sits below it, and optional panels (pages, bookmarks, form fields) dock at the sides. Panels are resizable, and modals are used only for decisions that interrupt the work: saving before close, managing pages, or signing.

Confirmed rejections: cloud upload tools. No bright gradients, no upload-first flow, and no marketing banners that imply the file leaves the machine. Upload-style framing contradicts the offline promise in PRODUCT.md.

**Key Characteristics:**
- Two fixed toolbar rows (44px and 38px) above the document, and a 32px status bar below it.
- Tonal layering for depth, with light and dark modes that flip every surface value.
- One functional blue accent, used only for working state.
- Pages render on pure white in both modes, so the document reads the same either way.
- Dense controls with compact spacing, sized for long sessions.

## Colors

The palette is a cool neutral base with one functional blue accent, plus a warm amber reserved for warnings and review. Every surface value flips between light and dark mode; the page stays white.

### Primary
- **Workbench Blue** (#146bca light / #5aa2e8 dark): the single working accent. It colours the PDFree wordmark, the focus and selection outlines around form fields, and the drag and selection overlays. Use it for working state only, never as decoration.

### Secondary
- **Page White** (#ffffff): the document page surface. It stays white in dark mode so the page reads as paper.

### Neutral
- **Harbour Canvas** (#e6ebf1 light / #0e1622 dark): the stage behind the page, where the document sits.
- **Cool Paper** (#f5f7fa light / #0b1220 dark): the window background.
- **Panel Grey** (#ffffff light / #16202e dark): the surface for toolbars, status bar, dialogs and search. The alt variant (#f4f6f9 light / #111a26 dark) is for side panels.
- **Hairline Slate** (#dbe3ec light / #2b3a4d dark): 1px borders and dividers.
- **Ink Navy** (#223b54 light / #e6edf5 dark): primary text.
- **Slate Secondary** (#526171 light / #9db0c3 dark): secondary text, hints and status.
- **Scrim** (#172b4d70): the dimmed backdrop behind modal dialogs.

### Warning
- **Amber Notice** (#805715 text / #fff8e8 background on light; #f0c36d text / #3a2f14 background on dark): unsaved state, file messages, and password and error prompts.
- **Review Ochre** (#d68c00): the border on automatically detected form fields that need review.

### Annotation and selection
- **Highlight Yellow** (#ffda326b): text highlights, at 42% alpha.
- **Note Yellow** (#ffe475): the sticky-note marker.
- **Selection Blue** (#3a8de65c): the selected form field and selected text overlay.

### Named Rules
**The Page Is Always White Rule.** The document page is #ffffff in both modes. Dark mode darkens the chrome and the canvas, never the page.

**The Working Blue Rule.** Workbench Blue marks what the user is working on: focus, selection, and the wordmark. It never marks a decorative element or a passive state.

## Typography

**Display Font:** the Slint platform default sans-serif. No family is declared in the source, so none is recorded here.
**Body Font:** the same platform default.
**Label/Mono Font:** none. Caveat (in `assets/signatures/Caveat.ttf`) is used only for the typed visual signature preview.

**Character:** a single system sans-serif, kept plain, with hierarchy built from weight and a small set of sizes rather than from pairing faces.

### Hierarchy
- **Display** (600, 25px): the empty-state heading, shown when no document is open.
- **Title** (700, 20px): the panel titles, such as the Form panel heading.
- **Dialog title** (600, 21px): modal titles, such as "Save your changes?" and "Manage pages".
- **Wordmark** (700, 19px): the PDFree name in the first toolbar row.
- **Body** (400, 14px): buttons, labels, and field values.
- **Label** (400, 12px): status text, zoom readout, hints and small notices.

### Named Rules
**The Two Sizes Rule.** Body and label carry almost all of the interface. Use display and title only for the empty state and panel headings. Do not add a third intermediate size.

## Layout

The window opens at 1180 by 850 and cannot shrink below 680 by 480. The layout is fixed from the top down:

1. Toolbar row one, 44px: the wordmark, file name, unsaved indicator, file actions (Open, Save, Save as, Export, Print) and the dark-mode toggle.
2. Toolbar row two, 38px: panel toggles (Pages, Bookmarks, Form), page tools, annotation tools, undo and redo, and Find.
3. A 30px notice strip, shown only during a rename or file message.
4. The main area: optional side panels at the left and right, with the viewport taking the remaining width.
5. The status bar, 32px: page navigation, the status message, zoom and view mode.

Side panels are resizable from their inner edge:
- Pages: 180px default, 140–360px.
- Bookmarks: 230px default, 160–420px.
- Form: 300px default, 240–480px.

The continuous view stacks pages vertically. Thumbnails are 148 by 184px in a 196px pitch. The Fit width / Fit page toggle appears only at 900px width and above.

Spacing follows a 4px base: 4, 8, 12 and 16px, plus 20px for modal padding.

## Elevation & Depth

Depth is tonal. Surfaces step in value (window background, panel alt, panel grey, canvas) to show their layers, and the system uses no drop shadows. The source separates surfaces with 1px borders in addition to tonal steps; the borders are the current implementation and remain in place until a later refinement pass. Modal dialogs are separated from the document by the dimmed scrim, not by a shadow.

### Shadow Vocabulary
None. The source defines no box shadows. Do not add one to lift a dialog or a panel; use a tonal step or the scrim instead.

### Named Rules
**The Tonal Step Rule.** Depth is a change in surface value, not a shadow. A panel is one step lighter or darker than the layer beneath it.

## Shapes

Two corner radii cover the system. The 8px radius is for surfaces that float above the document: the floating search bar, modal dialogs, and the page-picker and context menus. The 4px radius is for small marks: highlight and note markers and the sticky-note tag.

Borders are 1px in the border colour. The page itself is a sharp-cornered rectangle, as a printed sheet is.

## Components

Each component leads with its character, then its current state in the source.

**Target feel: tactile and confident.** The target is controls with a clear, physical response on hover and press. The current implementation does not yet deliver this: buttons are the Slint standard widgets styled only by the platform palette. The target is recorded here so later refinement has a direction; it is not a description of what ships today.

### Buttons
- **Shape:** the Slint standard button, with its default corner.
- **Primary:** no custom fill. Buttons take the platform palette. The working accent is not applied to buttons yet.
- **Hover / Focus:** the standard widget states. Focus is visible through the platform style. Any custom hover or press response belongs to the target feel above.
- **Icons:** the current buttons carry emoji glyphs in their labels (for example, the open, save, print and dark-mode buttons). These are the incumbent state and are a known weakness. The target is a single monoline icon set, to replace the emoji in a later pass.

### Toolbar rows
- **Style:** a Panel Grey surface, 44px and 38px tall, with 1px Hairline Slate dividers between groups.
- **Groups:** file actions in the first row; panel toggles, page tools and annotation tools in the second, separated by vertical dividers.
- **States:** disabled buttons are shown when no document is open or a file operation is busy.

### Floating search
- **Style:** a Panel Grey surface, 44px tall, 8px radius, 1px border, centred at the top of the viewport and at most 580px wide.
- **Behaviour:** it floats over the document and does not take a toolbar row.

### Modal dialogs
- **Style:** a Panel Grey surface, 8px radius, 20px padding, centred over a Scrim.
- **Sizes:** the note and save dialogs at 520px wide; the highlight dialog at 240px tall; the page tools dialog at up to 600 by 440px.
- **Decisions:** Save, Discard and Cancel for unsaved changes; Save note, Delete and Close for notes.

### Form fields and overlays
- **Style:** each detected or authored field is drawn as a tinted overlay with a 1px Workbench Blue outline. The selected field has a 2px outline and a move grip and a resize corner in Workbench Blue.
- **Review:** automatically detected fields that need review use a Review Ochre outline.
- **Editing:** the selected field gets an inline editor on the page. Enter, Tab or clicking away commits the value.

### Status bar
- **Style:** a Panel Grey surface, 32px tall, with the page readout on the left and zoom and view mode on the right.

### Page picker and context menu
- **Style:** a Panel Grey surface, 8px radius, 1px border. The context menu is 230px wide.

## Do's and Don'ts

### Do:
- **Do** keep the page #ffffff in both modes (see The Page Is Always White Rule).
- **Do** reserve Workbench Blue for focus, selection and the wordmark (see The Working Blue Rule).
- **Do** create depth with a tonal step between surfaces, never with a drop shadow.
- **Do** keep body text at 14px and labels at 12px, and use display or title sizes only for headings.
- **Do** make a new panel resizable from its inner edge, within its documented min and max width.
- **Do** keep modal decisions to the smallest set of buttons the choice needs.

### Don't:
- **Don't** add drop shadows to panels, dialogs or the page. Tonal steps and the scrim do this job.
- **Don't** use gradients or a bright marketing palette. The system is cool and functional.
- **Don't** imply that a file is uploaded or leaves the machine, in copy or in visuals (see the cloud upload anti-reference in Overview).
- **Don't** use the amber Warning colours for decoration. They mark unsaved state, file messages, errors and review.
- **Don't** add a third intermediate type size.
- **Don't** add new emoji icons. New icons follow the monoline target in Buttons.
