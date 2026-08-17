use serde::Serialize;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose};
use tauri::path::BaseDirectory;
use tauri::Manager;
use walkdir::WalkDir;

#[derive(Serialize, Clone)]
struct TreeNode {
    name: String,
    path: String,
    is_dir: bool,
    children: Vec<TreeNode>,
}

fn has_invalid_path_parts(relative_path: &str) -> bool {
    let p = Path::new(relative_path);
    p.components().any(|c| matches!(c, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
}

// Returns the bundled CSS themes directory.
fn css_themes_root(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    if cfg!(debug_assertions) {
        // CARGO_MANIFEST_DIR is src-tauri/ at compile time; go up one level.
        return Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("themes"));
    }
    // Matches the "../themes" entry in tauri.conf.json > bundle > resources,
    // which Tauri places under `$RESOURCE/_up_/themes`.
    app.path()
        .resolve("../themes", BaseDirectory::Resource)
        .map_err(|e| format!("Unable to resolve bundled themes directory: {e}"))
}

fn build_tree(root: &Path, current: &Path) -> Result<TreeNode, String> {
    let rel = current
        .strip_prefix(root)
        .unwrap_or(current)
        .to_string_lossy()
        .replace('\\', "/");

    let name = current
        .file_name()
        .map(|v| v.to_string_lossy().to_string())
        .unwrap_or_else(|| String::from("slides"));

    let mut node = TreeNode {
        name,
        path: if rel.is_empty() { String::from(".") } else { rel },
        is_dir: current.is_dir(),
        children: Vec::new(),
    };

    if current.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(current)
            .map_err(|e| format!("Failed to read directory {}: {e}", current.display()))?
            .filter_map(Result::ok)
            .collect();

        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();

            if path.is_dir() {
                node.children.push(build_tree(root, &path)?);
            } else if path.extension().map(|x| x == "md").unwrap_or(false) {
                let rel = path
                    .strip_prefix(root)
                    .map_err(|e| format!("Invalid path: {e}"))?
                    .to_string_lossy()
                    .replace('\\', "/");
                let file_name = path
                    .file_name()
                    .map(|v| v.to_string_lossy().to_string())
                    .unwrap_or_default();
                node.children.push(TreeNode {
                    name: file_name,
                    path: rel,
                    is_dir: false,
                    children: Vec::new(),
                });
            }
        }
    }

    Ok(node)
}

#[tauri::command]
fn get_slides_tree(slides_dir: String) -> Result<TreeNode, String> {
    let root = PathBuf::from(&slides_dir)
        .canonicalize()
        .map_err(|e| format!("Invalid slides directory: {e}"))?;
    if !root.is_dir() {
        return Err(format!("Not a directory: {}", root.display()));
    }
    build_tree(&root, &root)
}

#[tauri::command]
fn read_slide(slides_dir: String, relative_path: String) -> Result<String, String> {
    if has_invalid_path_parts(&relative_path) {
        return Err(String::from("Invalid path"));
    }
    let root = PathBuf::from(&slides_dir)
        .canonicalize()
        .map_err(|e| format!("Invalid directory: {e}"))?;
    let path = root.join(&relative_path)
        .canonicalize()
        .map_err(|_| String::from("File not found"))?;
    if !path.starts_with(&root) {
        return Err(String::from("Access denied"));
    }
    if path.extension().map(|x| x == "md").unwrap_or(false) {
        fs::read_to_string(&path).map_err(|e| format!("Failed to read: {e}"))
    } else {
        Err(String::from("Only .md files are allowed"))
    }
}

fn extract_import_url(line: &str) -> Option<String> {
    let s = line.trim();
    if !s.starts_with("@import") {
        return None;
    }
    let after = s["@import".len()..].trim();
    let inner = if let Some(rest) = after.strip_prefix("url(") {
        rest.trim_end_matches(';').trim_end_matches(')').trim()
    } else {
        after.trim_end_matches(';').trim()
    };
    let path = inner.trim_matches(|c| c == '\'' || c == '"').to_string();
    if path.is_empty() { None } else { Some(path) }
}

// Reads a CSS file and inlines one level of @import url(...) from the same directory.
fn inline_css_imports(root: &Path, css_path: &Path) -> Result<String, String> {
    let content = fs::read_to_string(css_path)
        .map_err(|e| format!("Failed to read CSS: {e}"))?;
    let parent = css_path.parent().unwrap_or(css_path);
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        if let Some(rel) = extract_import_url(line) {
            let imported_path = parent.join(&rel);
            if let Ok(canon) = imported_path.canonicalize() {
                if canon.starts_with(root) {
                    if let Ok(imported) = fs::read_to_string(&canon) {
                        out.push_str(&imported);
                        out.push('\n');
                        continue;
                    }
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    Ok(out)
}

#[tauri::command]
fn list_css_themes(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let root = css_themes_root(&app)?;
    let mut files = Vec::new();

    for entry in WalkDir::new(&root).min_depth(1).max_depth(2).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if p.is_file() && p.extension().map(|x| x == "css").unwrap_or(false) {
            if let Ok(rel) = p.strip_prefix(&root) {
                files.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    files.sort();
    Ok(files)
}

#[tauri::command]
fn read_css_theme(app: tauri::AppHandle, relative_path: String) -> Result<String, String> {
    if has_invalid_path_parts(&relative_path) {
        return Err(String::from("Invalid path"));
    }
    let root = css_themes_root(&app)?
        .canonicalize()
        .map_err(|e| format!("Themes directory not accessible: {e}"))?;
    let path = root.join(&relative_path)
        .canonicalize()
        .map_err(|_| String::from("Theme file not found"))?;
    if !path.starts_with(&root) {
        return Err(String::from("Access denied"));
    }
    if path.extension().map(|x| x == "css").unwrap_or(false) {
        inline_css_imports(&root, &path)
    } else {
        Err(String::from("Only .css files are allowed"))
    }
}

#[tauri::command]
fn read_documentation(app: tauri::AppHandle) -> Result<String, String> {
    let doc_paths = if cfg!(debug_assertions) {
        // In dev mode, read from project root
        vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("DOCUMENTATION.md")]
    } else {
        // In production, read from bundled resources
        let resource_dir = app.path()
            .resolve(".", BaseDirectory::Resource)
            .map_err(|e| format!("Unable to resolve documentation resources: {e}"))?;
        vec![
            resource_dir.join("DOCUMENTATION.md"),
            resource_dir.join("_up_").join("DOCUMENTATION.md"),
        ]
    };

    for doc_path in doc_paths {
        if let Ok(markdown) = fs::read_to_string(&doc_path) {
            return Ok(markdown);
        }
    }

    Err(String::from("Documentation file not found"))
}

#[tauri::command]
fn load_image(slides_dir: String, md_relative_path: String, img_src: String) -> Result<String, String> {
    // Block absolute and remote URLs; only serve relative local images.
    if img_src.starts_with('/') || img_src.contains("://") {
        return Err(String::from("Only relative paths are supported"));
    }
    let root = PathBuf::from(&slides_dir)
        .canonicalize()
        .map_err(|e| format!("Invalid directory: {e}"))?;
    let md_abs = root.join(&md_relative_path);
    let md_dir = md_abs.parent().unwrap_or(&root);
    let img_abs = md_dir.join(&img_src)
        .canonicalize()
        .map_err(|_| String::from("Image not found"))?;
    if !img_abs.starts_with(&root) {
        return Err(String::from("Access denied"));
    }
    let ext = img_abs.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let mime = match ext.as_str() {
        "png"  => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif"  => "image/gif",
        "svg"  => "image/svg+xml",
        "webp" => "image/webp",
        "bmp"  => "image/bmp",
        _ => return Err(format!("Unsupported image format: {ext}")),
    };
    let bytes = fs::read(&img_abs).map_err(|e| format!("Failed to read image: {e}"))?;
    let b64 = general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{mime};base64,{b64}"))
}

#[tauri::command]
fn open_print_preview(html: String) -> Result<(), String> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Unable to create preview filename: {e}"))?
        .as_millis();
    let path = std::env::temp_dir().join(format!("markdown-slide-viewer-{stamp}.html"));
    fs::write(&path, html).map_err(|e| format!("Unable to write print preview: {e}"))?;

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(&path).status();
    #[cfg(target_os = "windows")]
    let result = Command::new("cmd").args(["/C", "start", "", &path.to_string_lossy()]).status();
    #[cfg(all(unix, not(target_os = "macos")))]
    let result = Command::new("xdg-open").arg(&path).status();

    match result {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("Unable to open print preview (exit status {status})")),
        Err(e) => Err(format!("Unable to open print preview: {e}")),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_slides_tree,
            read_slide,
            load_image,
            list_css_themes,
            read_css_theme,
            read_documentation,
            open_print_preview
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
