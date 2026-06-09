//! Real-file import tests: write actual .txt/.md/.docx files to disk and run
//! the real `extract_text` + import path (not just the pure helpers). This is
//! the end-to-end coverage the unit tests were missing.

use openprompter_rs_tauri::adapters::document::extract_text;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn extract_real_txt_file() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("note.txt");
    std::fs::write(&p, "Plain line one\nLine two").unwrap();
    let text = extract_text(&p).unwrap();
    assert!(text.contains("Plain line one"));
    assert!(text.contains("Line two"));
}

#[test]
fn extract_real_markdown_file() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("speech.md");
    std::fs::write(
        &p,
        "# My Speech\n\nHello **everyone**, welcome.\n\n- point one\n- point two\n\nSee [docs](http://x).\n",
    )
    .unwrap();

    let text = extract_text(&p).unwrap();
    assert!(!text.trim().is_empty(), "markdown extraction was empty");
    assert!(text.contains("My Speech"));
    assert!(text.contains("Hello everyone, welcome."));
    assert!(text.contains("point one"));
    assert!(text.contains("docs"));
    assert!(!text.contains('#'));
    assert!(!text.contains('*'));
}

#[test]
fn extract_real_docx_file() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("doc.docx");

    // Minimal valid .docx: a zip containing word/document.xml.
    let file = std::fs::File::create(&p).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let opts: zip::write::FileOptions<()> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file("word/document.xml", opts).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?><w:document xmlns:w="x"><w:body>
            <w:p><w:r><w:t>Hello from docx</w:t></w:r></w:p>
            <w:p><w:r><w:t>Second paragraph</w:t></w:r></w:p>
        </w:body></w:document>"#,
    )
    .unwrap();
    zip.finish().unwrap();

    let text = extract_text(&p).unwrap();
    assert!(text.contains("Hello from docx"), "got: {text:?}");
    assert!(text.contains("Second paragraph"), "got: {text:?}");
}

#[test]
fn unsupported_extension_is_error_not_panic() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("data.rtf");
    std::fs::write(&p, "x").unwrap();
    assert!(extract_text(&p).is_err());
}
