# Product

<!-- impeccable:product-schema 1 -->

## Platform

desktop

Native Windows 11 x64 application (Rust + Slint UI, MuPDF rendering). Recorded as `desktop` per the user's answer; the schema's enumerated values (`web`, `ios`, `android`, `adaptive`) do not cover it. Mobile and web design guidance does not apply.

## Users

Power users and developers who want a fast, lightweight, native PDF viewer and editor on Windows, with no install-time services, accounts, or telemetry.

## Product Purpose

PDFree is an offline PDF viewer, form editor, and annotation tool. It reads real PDFs in a virtualized continuous viewport and supports navigation, search, text selection, highlights and notes, page operations (rotate, move, delete, insert, merge, extract, split), AcroForm filling and authoring, offline field detection, visual and digital signatures, and printing.

Success means reading and editing real PDFs quickly and faithfully, entirely on the local machine. Performance targets are goals, not established claims.

## Positioning

Offline and private by design: no network services, no telemetry, no JavaScript execution, and no cloud requirement. This is the mechanism a cloud-based PDF editor cannot truthfully copy.

## Operating Context

- Distributed as a portable ZIP from GitHub Releases, with a SHA-256 checksum and provenance attestation.
- Registered per Windows account with `register-pdfree.ps1`, without admin rights or changing the current default.
- Opens PDFs from disk, via the command line, or through the native Open dialog. Print hands the copy to the Windows printer through `ShellExecuteW`.
- Built from source with Rust 1.97.1 MSVC and Visual Studio C++ Build Tools.

## Capabilities and Constraints

- Viewing: continuous, single-page, and facing views; zoom 25–400%; fit width and page; case-insensitive search; lazy thumbnails; outlines; internal and HTTP/HTTPS links; cross-page text copy up to 16 pages.
- Annotation and history: highlights (yellow, green, blue, pink) and sticky notes; shared undo and redo (200 commands, 128 MiB of snapshots); Save as an in-place commit; Save As to a new file only.
- Pages: rotate, move, delete, insert blank, insert or merge up to 16 unpassworded PDFs, extract, and split. Editing supports up to 100,000 pages.
- Forms: existing AcroForm filling; manual form authoring; date and number fields; offline automatic field detection (English OCR, 100 pages per run); visual signatures; digital signatures (Windows, one signer, trust not evaluated).
- Out of scope in the prototype: secure redaction, searchable-text OCR, compression, conversion, and a PDFium fallback.
- Terminology to keep consistent: "Extract" and "Split" are not secure redaction. Page extraction and deletion can leave content in metadata or other references.
- Undecided: whether any non-Windows build is planned. Print is Windows-only; other platforms report printing as unsupported.
- Undecided: accessibility requirements have not been captured.

## Brand Commitments

- Name: PDFree. Logo: `assets/pdfree-logo.svg`.
- Offline and private operation is a binding core promise, confirmed by the user. It should not be softened into a secondary feature.
- No voice guidelines are recorded yet.

## Evidence on Hand

- Product and technical docs: `README.md`, `docs/feature-matrix.md`, `docs/architecture.md`, `docs/backlog.md`, `docs/manual-validation.md`, `docs/forms.md`, `docs/reader-navigation-design.md`.
- Benchmarks and validation: `benchmarks/results/REPORT.md`, `benchmarks/results/reader-tools.md`, `benchmarks/results/annotations.md`, `benchmarks/results/page-tools.md`.
- Test corpus with provenance and hashes: `tests/corpus/manifest.json`.
- Absences future work must not fabricate: no user research, testimonials, customer names, press, or competitive benchmark wins are on record. Physical presentation timing, post-reboot cold trials, and broad independent-reader comparison are still pending, per the README.

## Product Principles

1. Offline and private by construction: never add a network dependency, telemetry, or cloud step to a core path.
2. Native responsiveness on large documents: rendering and navigation must stay fast, and every speed claim needs measured evidence.
3. Faithful to the file: preserve links, forms, annotations, and encryption across saves. Reject incompatible edits transactionally rather than silently damaging a document.
4. Honest about scope: label experimental and unsupported features clearly. Never imply secure redaction, searchable OCR, or conversion exists when it does not.
5. Keyboard-first and dense for daily PDF work: predictable controls and shortcuts (Ctrl+O, Ctrl+F, Ctrl+S, Ctrl+Z, PgUp/PgDn) over decoration.
