# Markdown Slide Viewer (Tauri + Svelte)

Desktop markdown slide viewer with bundled themes.

> **📖 For detailed usage instructions and how to prepare slides, see [DOCUMENTATION.md](DOCUMENTATION.md)**

## Features

- Two-pane layout with toggleable left file tree
- Markdown slide rendering split by horizontal rules (`---`, `***`, `___`)
- Theme selection from bundled `.css` files
- Bundled `themes/` folder packed into the app

## Local Development

```bash
cd markdown-slide-viewer
npm install
npm run tauri dev
```

## Build

```bash
cd markdown-slide-viewer
npm install
npm run tauri build
```

Windows bundle output (on Windows runner or machine):

- `src-tauri/target/release/bundle/nsis/*.exe`

## GitHub Actions

Workflow file:

- `.github/workflows/build-markdown-slide-viewer-windows.yml`

It builds a Windows executable and uploads artifacts.
