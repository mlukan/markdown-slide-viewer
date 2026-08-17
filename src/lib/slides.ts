import MarkdownIt from "markdown-it";

const md = new MarkdownIt({ html: true, linkify: true, breaks: false });

// Renders markdown, splits on <hr> and wraps each segment in <div class="slide">.
export function renderMarkdown(markdownText: string): string {
  const html = md.render(markdownText);
  const segments = html
    .split(/<hr\s*\/?>/i)
    .map((s) => s.trim())
    .filter((s) => s.length > 0);
  return segments.length > 0
    ? segments.map((s) => `<div class="slide">${s}</div>`).join("\n")
    : "";
}
