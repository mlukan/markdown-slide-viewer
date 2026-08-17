<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onDestroy, tick } from "svelte";
  import { renderMarkdown } from "./lib/slides";
  import TreeItem from "./lib/TreeItem.svelte";

  type TreeNode = {
    name: string;
    path: string;
    is_dir: boolean;
    children: TreeNode[];
  };

  let sidebarOpen = true;
  let tree: TreeNode | null = null;
  let cssThemes: string[] = [];
  let selectedTheme = "telekom-pastel-slides-strict.css";
  let selectedFile = "";
  let slidesDir: string | null = null;
  let renderedSlides = "";
  let error = "";
  let writeEl: HTMLElement;

  let expanded = new Set<string>();

  // Reactive props so TreeItem re-renders when selectedFile or expanded changes.
  $: isActive = (path: string) => selectedFile === path;
  $: isOpen = (path: string) => path === "." || expanded.has(path);

  // Inject theme CSS into <head> so it applies globally, not scoped to a div.
  let themeEl: HTMLStyleElement | null = null;
  function applyThemeCss(css: string) {
    if (!themeEl) {
      themeEl = document.createElement("style");
      themeEl.id = "injected-theme";
      document.head.appendChild(themeEl);
    }
    themeEl.textContent = css;
  }
  onDestroy(() => themeEl?.remove());
  let _themeCss = "";
  $: applyThemeCss(_themeCss);

  async function init() {
    try {
      cssThemes = await invoke<string[]>("list_css_themes");
      if (!cssThemes.includes(selectedTheme) && cssThemes.length > 0) {
        selectedTheme = cssThemes[0];
      }
      await loadTheme();
    } catch (e) {
      error = String(e);
    }
  }

  async function selectFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (typeof selected === "string") {
      slidesDir = selected;
      tree = null;
      selectedFile = "";
      error = "";
      renderedSlides = "<p>Loading…</p>";
      try {
        tree = await invoke<TreeNode>("get_slides_tree", { slidesDir });
        const first = findFirstMarkdown(tree);
        if (first) {
          await selectFile(first.path);
        } else {
          renderedSlides = "<p>No markdown files found in this folder.</p>";
        }
      } catch (e) {
        error = String(e);
      }
    }
  }

  function findFirstMarkdown(node: TreeNode | null): TreeNode | null {
    if (!node) return null;
    if (!node.is_dir && node.path.endsWith(".md")) return node;
    for (const child of node.children) {
      const found = findFirstMarkdown(child);
      if (found) return found;
    }
    return null;
  }

  async function loadTheme() {
    if (!selectedTheme) return;
    _themeCss = await invoke<string>("read_css_theme", { relativePath: selectedTheme });
  }

  async function selectFile(path: string) {
    if (!slidesDir) return;
    selectedFile = path;
    const markdown = await invoke<string>("read_slide", { slidesDir, relativePath: path });
    renderedSlides = renderMarkdown(markdown);
    await tick();
    await fixLocalImages(path);
    writeEl?.scrollTo(0, 0);
    writeEl?.focus({ preventScroll: true });
  }

  async function fixLocalImages(mdRelativePath: string) {
    if (!writeEl || !slidesDir) return;
    const imgs = writeEl.querySelectorAll<HTMLImageElement>("img");
    await Promise.all(Array.from(imgs).map(async (img) => {
      const src = img.getAttribute("src") ?? "";
      if (!src || src.startsWith("data:") || src.startsWith("http") || src.startsWith("asset:")) return;
      try {
        img.src = await invoke<string>("load_image", { slidesDir, mdRelativePath, imgSrc: src });
      } catch { /* image stays broken rather than crashing */ }
    }));
  }

  function toggleNode(path: string) {
    if (expanded.has(path)) expanded.delete(path);
    else expanded.add(path);
    expanded = expanded; // reassign to trigger Svelte reactivity
  }

  function onOpenFile(event: CustomEvent<string>) {
    selectFile(event.detail).catch((e) => (error = String(e)));
  }

  function onToggle(event: CustomEvent<string>) {
    toggleNode(event.detail);
  }

  function onThemeChange() {
    loadTheme().catch((e) => (error = String(e)));
  }

  async function openDocumentation() {
    try {
      const markdown = await invoke<string>("read_documentation");
      renderedSlides = renderMarkdown(markdown);
      selectedFile = "Documentation";
      await tick();
      writeEl?.scrollTo(0, 0);
      writeEl?.focus({ preventScroll: true });
    } catch (e) {
      error = String(e);
    }
  }

  async function printSlides() {
    if (!renderedSlides) return;
    const images = writeEl?.querySelectorAll<HTMLImageElement>("img") ?? [];
    await Promise.all(Array.from(images).map(async (img) => {
      if (!img.complete) {
        await new Promise<void>((resolve) => {
          img.addEventListener("load", () => resolve(), { once: true });
          img.addEventListener("error", () => resolve(), { once: true });
        });
      }
      try {
        await img.decode();
      } catch { /* the print preview can still show the image if decoding is deferred */ }
    }));
    const printCss = `
      @page { margin: 0; }
      html, body { margin: 0; background: white; }
      body { color: #222; }
      #write { margin: 0; padding: 0; }
      .slide {
        min-height: 0 !important;
        margin: 0 !important;
        padding: 1cm !important;
        border: 0 !important;
        border-radius: 0 !important;
        box-shadow: none !important;
        break-inside: avoid;
        break-after: page;
      }
      .slide:last-child { break-after: auto; }
    `;
    const printContent = writeEl?.innerHTML ?? renderedSlides;
    const printHtml = `<!doctype html><html><head><meta charset="utf-8"><title>${selectedFile}</title><style>${_themeCss}${printCss}</style></head><body><main id="write">${printContent}</main></body></html>`;
    try {
      await invoke("open_print_preview", { html: printHtml });
    } catch (e) {
      error = String(e);
    }
  }

  init();
</script>

<div class="shell" class:sidebar-collapsed={!sidebarOpen}>
  <aside class="sidebar">
    <div class="sidebar-controls">
      <button class="full-width" on:click={selectFolder}>Open Folder…</button>
      <label class="theme-label">
        Theme
        <select bind:value={selectedTheme} on:change={onThemeChange}>
          {#each cssThemes as css}
            <option value={css}>{css}</option>
          {/each}
        </select>
      </label>
    </div>
    <div class="sidebar-files">
      {#if tree}
        <ul class="tree">
          <TreeItem
            node={tree}
            {expanded}
            {isOpen}
            {isActive}
            on:openFile={onOpenFile}
            on:toggle={onToggle}
          />
        </ul>
      {:else}
        <p class="muted">No folder selected</p>
      {/if}
    </div>
    <div class="sidebar-footer">
      <button class="doc-button" on:click={openDocumentation} aria-label="Open documentation">
        <span aria-hidden="true">📖</span>
        Documentation
      </button>
    </div>
  </aside>

  <section class="main">
    <header class="toolbar">
      <button class="menu-toggle" on:click={() => (sidebarOpen = !sidebarOpen)}>☰</button>
      <span class="muted current-file">{selectedFile}</span>
      <button class="print-button" on:click={printSlides} disabled={!renderedSlides}>Print</button>
    </header>

    <div class="viewer-wrap">
      <main id="write" tabindex="-1" bind:this={writeEl}>{@html renderedSlides}</main>
    </div>
  </section>
</div>

{#if error}
  <pre class="error">{error}</pre>
{/if}
