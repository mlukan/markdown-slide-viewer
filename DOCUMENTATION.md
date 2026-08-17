# Markdown Slide Viewer - Usage & Documentation

A lightweight desktop application for presenting markdown files as slide presentations. Perfect for developers, educators, and presenters who prefer writing content in markdown.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [User Interface](#user-interface)
- [Preparing Slides](#preparing-slides)
- [Slide Syntax](#slide-syntax)
- [Themes](#themes)
- [Tips & Best Practices](#tips--best-practices)

## Features

- **Two-pane layout** with toggleable left file tree for easy navigation
- **Markdown slide rendering** split by horizontal rules
- **Multiple themes** for customized presentation appearance
- **Live preview** of markdown files as slides
- **Keyboard navigation** for seamless presentation control
- **Cross-platform** support (Windows, macOS, Linux)

## Getting Started

1. **Open the application** - Launch Markdown Slide Viewer from your applications folder or desktop shortcut
2. **Browse files** - Use the left file tree panel to navigate to your markdown files
3. **Select a file** - Click on any `.md` file to load it as slides
4. **Choose a theme** - Select your preferred theme from the theme menu (top toolbar)
5. **Present** - Navigate through slides using keyboard shortcuts or mouse controls

## User Interface

### Main Layout

```
┌─────────────────────────────────────────┐
│  Toolbar (Theme selector, controls)     │
├──────────────┬──────────────────────────┤
│ File Tree    │                          │
│ (Toggleable) │    Slide Display Area    │
│              │                          │
│              │                          │
└──────────────┴──────────────────────────┘
```

### Toolbar

- **Theme Selector** - Dropdown menu to choose from available themes
- **File Tree Toggle** - Show/hide the left file navigation panel

### File Tree Panel

- Browse your file system
- Select markdown files to load as presentations
- Provides quick access to multiple presentation files

### Slide Display Area

- Renders markdown content as formatted slides
- Shows one slide at a time
- Supports all common markdown formatting

## Preparing Slides

### Basic Setup

1. Create a `.md` (markdown) file in any text editor
2. Write your content using markdown syntax
3. Separate slides using horizontal rules: `---`, `***`, or `___`
4. Open the file in Markdown Slide Viewer

### Recommended Markdown Editors

You can prepare slides in any markdown editor:

- **VS Code** with markdown preview extension
- **Sublime Text** with markdown plugins
- **Atom** with markdown preview
- **Typora** (WYSIWYG markdown editor)
- **iA Writer** (clean, focused writing)
- **Obsidian** (note-taking with markdown)
- **Simple text editors** (Notepad++, Vim, etc.)

## Slide Syntax

### Separating Slides

Use any of these horizontal rules to create slide breaks:

```markdown
---
```

```markdown
***
```

```markdown
___
```

### Basic Markdown Formatting

Markdown Slide Viewer supports standard markdown syntax:

#### Headers

```markdown
# Heading 1
## Heading 2
### Heading 3
```

#### Text Formatting

```markdown
**Bold text**
*Italic text*
***Bold and italic***
~~Strikethrough~~
```

#### Lists

```markdown
- Bullet point
- Another point
  - Nested point

1. Numbered item
2. Another item
   1. Nested numbered item
```

#### Code

Inline code: `` `code` ``

Code blocks:
```markdown
```language
code block content
```
```

#### Links and Images

```markdown
[Link text](https://example.com)
![Image alt text](image.jpg)
```

#### Blockquotes

```markdown
> This is a blockquote
> Multiple lines are supported
```

#### Speaker Notes & Collapsible Content

Use HTML `<details>` and `<summary>` tags to create collapsible sections for speaker notes, additional information, or content you want to hide by default:

```html
<details>
<summary>Click to reveal speaker notes</summary>

This content is hidden by default and will only show when the user clicks the summary.

- Add speaker notes here
- Include talking points
- Reference additional resources
- Or any other hidden content

</details>
```

**Multi-line content example:**

```html
<details>
<summary>📝 Speaker Notes</summary>

**Key Points:**
- Emphasize the importance of this topic
- Pause here for questions
- Refer to the chart on slide 3

**Time allocation:** ~2 minutes for this section

**Demo setup:** Ensure terminal is ready

</details>
```

**Multiple collapsible sections on one slide:**

```markdown
# My Slide Title

Main visible content here.

<details>
<summary>Additional Context</summary>
Hidden supplementary information
</details>

<details>
<summary>Speaker Notes</summary>
Private notes for the presenter
</details>
```

This approach is useful for:
- **Presenter notes** - Keep talking points hidden during presentation
- **Source citations** - Reference materials hidden from audience
- **Extended explanations** - Detailed content that's optional
- **Q&A references** - Common questions and answers
- **Alternative explanations** - Different ways to explain complex topics

### Example Presentation Structure

```markdown
# Welcome to My Presentation

This is the title slide with an introduction.

---

## Main Topic

Here's the first section of content.

---

## Another Section

- Point one
- Point two
- Point three

---

## Code Example

```python
def hello():
    print("Hello, World!")
```

---

## Thank You!

Questions?
```

## Themes

The application includes several built-in CSS themes for customizing the presentation appearance:

- **github** - Clean GitHub-inspired theme
- **telekom** - Professional corporate theme
- **telekom-dark** - Dark mode version of telekom
- **telekom-pastel-slides** - Soft pastel colors
- **telekom-pastel-slides-strict** - Strict version of pastel theme
- **whitey** - Minimalist white theme
- **turtle-dark** - Dark nature-inspired theme
- **turtle-green** - Green nature-inspired theme

Select themes from the theme dropdown in the toolbar. Themes are applied instantly to the current presentation.

### Custom Themes

To add custom themes:

1. Create a `.css` file in the `themes/` folder
2. Restart the application
3. Your theme will appear in the theme selector

## Tips & Best Practices

### Content Organization

- **Keep slides focused** - One main idea per slide
- **Use short bullet points** - Avoid large blocks of text
- **Include visual breaks** - Use headers and spacing effectively
- **Limit items per slide** - Aim for 5-7 bullet points maximum

### Formatting Best Practices

- Use headers to structure content hierarchy
- Employ bold and italic formatting for emphasis
- Include code blocks for technical content
- Use blockquotes for important statements or quotes
- Keep line length reasonable for readability

### File Management

- Store all related markdown files in a single folder
- Use descriptive file names for easy navigation
- Keep the `themes/` folder organized
- Create separate `.md` files for different presentations

### Presentation Tips

- Test your slides with different themes before presenting
- Ensure code examples fit properly on slides
- Use consistent formatting throughout the presentation
- Preview slides on the display setup you'll use for presentation
- Keep backup copies of your markdown files

### Navigation

- Use arrow keys or mouse clicks to move between slides
- The file tree helps organize multiple presentations
- Switch themes on-the-fly during presentation prep

## Keyboard Shortcuts

Common navigation controls:
- **Next slide** - Arrow Right, Space, or Page Down
- **Previous slide** - Arrow Left, Backspace, or Page Up
- **Toggle file tree** - Usually via UI button

## Troubleshooting

### Slides not displaying correctly

- Verify horizontal rules use `---`, `***`, or `___`
- Check that the markdown syntax is valid
- Try a different theme to rule out CSS issues

### File tree not showing files

- Ensure you're in the correct directory
- Check file has `.md` extension
- Refresh or re-open the file tree

### Theme not applying

- Restart the application
- Ensure CSS file is in the `themes/` folder
- Check theme file syntax for CSS errors

## Support

For issues, feature requests, or questions about the Markdown Slide Viewer, please refer to the project repository or documentation.
