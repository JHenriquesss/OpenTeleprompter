//! Extract plain text from supported document formats for import.
//!
//! A teleprompter shows plain text, so every format is reduced to text:
//! - `.txt`              -> read as-is
//! - `.md` / `.markdown` -> light markdown strip (headings, emphasis, links…)
//! - `.pdf`              -> text layer extraction (`pdf-extract`)
//! - `.docx`            -> read `word/document.xml` from the zip, join `<w:t>`

use crate::domain::errors::AppError;
use std::io::Read;
use std::path::Path;

/// Extract plain text from `path`, dispatching on the file extension.
pub fn extract_text(path: &Path) -> Result<String, AppError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "txt" | "text" | "" => read_plain(path),
        "md" | "markdown" => Ok(markdown_to_text(&read_plain(path)?)),
        "pdf" => read_pdf(path),
        "docx" => read_docx(path),
        other => Err(AppError::InvalidInput(format!(
            "Unsupported file type: .{other} (supported: txt, md, pdf, docx)"
        ))),
    }
}

/// File extensions accepted by the import file picker / drag-and-drop.
pub fn supported_extensions() -> &'static [&'static str] {
    &["txt", "text", "md", "markdown", "pdf", "docx"]
}

/// True if `path`'s extension is one we can import.
pub fn is_supported(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    supported_extensions().contains(&ext.as_str())
}

fn read_plain(path: &Path) -> Result<String, AppError> {
    std::fs::read_to_string(path).map_err(|e| AppError::General(e.to_string()))
}

fn read_pdf(path: &Path) -> Result<String, AppError> {
    pdf_extract::extract_text(path)
        .map(|t| normalize_whitespace(&t))
        .map_err(|e| AppError::General(format!("Could not read PDF: {e}")))
}

fn read_docx(path: &Path) -> Result<String, AppError> {
    let file = std::fs::File::open(path).map_err(|e| AppError::General(e.to_string()))?;
    let mut zip =
        zip::ZipArchive::new(file).map_err(|e| AppError::General(format!("Invalid docx: {e}")))?;
    let mut xml = String::new();
    zip.by_name("word/document.xml")
        .map_err(|e| AppError::General(format!("docx missing document.xml: {e}")))?
        .read_to_string(&mut xml)
        .map_err(|e| AppError::General(e.to_string()))?;
    Ok(docx_xml_to_text(&xml))
}

/// Pull readable text out of a WordprocessingML body: `<w:t>` holds runs of
/// text, `<w:p>` is a paragraph (newline), `<w:br>`/`<w:tab>` are breaks.
fn docx_xml_to_text(xml: &str) -> String {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut out = String::new();
    let mut in_text = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"t" => in_text = true,
            Ok(Event::End(e)) if e.local_name().as_ref() == b"t" => in_text = false,
            Ok(Event::Text(t)) if in_text => {
                out.push_str(&t.unescape().unwrap_or_default());
            }
            Ok(Event::End(e)) if e.local_name().as_ref() == b"p" => out.push('\n'),
            Ok(Event::Empty(e)) => match e.local_name().as_ref() {
                b"br" => out.push('\n'),
                b"tab" => out.push('\t'),
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    out.trim_end().to_string()
}

/// Collapse the runs of blank lines / stray spaces that PDF extraction tends to
/// produce, without destroying paragraph breaks.
fn normalize_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut blank_run = 0;
    for line in text.lines() {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() {
            blank_run += 1;
            if blank_run <= 1 {
                out.push('\n');
            }
        } else {
            blank_run = 0;
            out.push_str(trimmed);
            out.push('\n');
        }
    }
    out.trim().to_string()
}

/// Light markdown -> plain text. Keeps prose and `[pause]`-style cues, strips
/// the common formatting syntax that would read oddly on a teleprompter.
fn markdown_to_text(md: &str) -> String {
    use regex::Regex;
    use std::sync::OnceLock;

    static IMG: OnceLock<Regex> = OnceLock::new();
    static LINK: OnceLock<Regex> = OnceLock::new();
    static EMPH: OnceLock<Regex> = OnceLock::new();
    static CODE: OnceLock<Regex> = OnceLock::new();

    let img = IMG.get_or_init(|| Regex::new(r"!\[([^\]]*)\]\([^)]*\)").unwrap());
    let link = LINK.get_or_init(|| Regex::new(r"\[([^\]]+)\]\([^)]*\)").unwrap());
    let emph = EMPH.get_or_init(|| Regex::new(r"(\*\*|\*|__|_|~~)").unwrap());
    let code = CODE.get_or_init(|| Regex::new(r"`+").unwrap());

    let mut out = String::new();
    for raw in md.lines() {
        let mut line = raw.to_string();

        // Skip fenced code fences and horizontal rules.
        let t = line.trim();
        if t.starts_with("```") || t == "---" || t == "***" || t == "___" {
            continue;
        }

        // Strip leading block syntax: headings, blockquotes, list markers.
        let mut s = line.trim_start();
        s = s.trim_start_matches('#').trim_start();
        s = s.trim_start_matches('>').trim_start();
        let bullet = s
            .strip_prefix("- ")
            .or_else(|| s.strip_prefix("* "))
            .or_else(|| s.strip_prefix("+ "));
        if let Some(rest) = bullet {
            s = rest;
        }
        line = s.to_string();

        // Inline replacements.
        line = img.replace_all(&line, "$1").to_string();
        line = link.replace_all(&line, "$1").to_string();
        line = emph.replace_all(&line, "").to_string();
        line = code.replace_all(&line, "").to_string();

        out.push_str(line.trim_end());
        out.push('\n');
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_strips_formatting() {
        let md =
            "# Title\n\nHello **world** and _you_.\n\n- item one\n- item two\n\n[link](http://x)\n";
        let txt = markdown_to_text(md);
        assert!(txt.contains("Title"));
        assert!(txt.contains("Hello world and you."));
        assert!(txt.contains("item one"));
        assert!(txt.contains("link"));
        assert!(!txt.contains('*'));
        assert!(!txt.contains('#'));
        assert!(!txt.contains("http"));
    }

    #[test]
    fn docx_xml_extracts_paragraph_text() {
        let xml = r#"<w:document><w:body>
            <w:p><w:r><w:t>First line</w:t></w:r></w:p>
            <w:p><w:r><w:t>Second </w:t><w:t>line</w:t></w:r></w:p>
        </w:body></w:document>"#;
        let txt = docx_xml_to_text(xml);
        assert!(txt.contains("First line"));
        assert!(txt.contains("Second line"));
    }

    #[test]
    fn unsupported_extension_errors() {
        assert!(extract_text(Path::new("foo.xlsx")).is_err());
    }

    #[test]
    fn supported_list_matches_is_supported() {
        assert!(is_supported(Path::new("a.PDF")));
        assert!(is_supported(Path::new("a.docx")));
        assert!(is_supported(Path::new("a.md")));
        assert!(!is_supported(Path::new("a.rtf")));
    }
}
