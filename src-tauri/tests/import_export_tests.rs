use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn can_read_text_file_content() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "Hello, world!").unwrap();

    let content = std::fs::read_to_string(file.path()).unwrap();
    assert!(content.contains("Hello, world!"));
}

#[test]
fn can_write_text_file_content() {
    let dir = std::env::temp_dir().join("openprompter-test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test_export.txt");

    std::fs::write(&path, "Test content").unwrap();
    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Test content");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn extract_title_from_filename() {
    let filename = "my_speech.txt";
    let title = filename.strip_suffix(".txt").unwrap_or(filename);
    assert_eq!(title, "my_speech");

    let filename_no_ext = "my_speech";
    let title = filename_no_ext
        .strip_suffix(".txt")
        .unwrap_or(filename_no_ext);
    assert_eq!(title, "my_speech");
}
