//! CLI and end-to-end integration tests for the `extractor_audio` binary.

use std::fs;
use std::process::Command as StdCommand;

use assert_cmd::Command;
use predicates::prelude::*;

fn bin() -> Command {
    Command::cargo_bin("extractor_audio").expect("binary should be built")
}

fn ffmpeg_available() -> bool {
    StdCommand::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
fn help_succeeds() {
    bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("extractor_audio"));
}

#[test]
fn invalid_format_fails() {
    bin()
        .args(["extract", "some_video.mp4", "-f", "xyz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("xyz"));
}

#[test]
fn missing_input_file_fails() {
    // Fails regardless of FFmpeg availability: either the missing-file check
    // or the up-front FFmpeg check reports failure with a non-zero exit code.
    let dir = tempfile::tempdir().unwrap();
    bin()
        .current_dir(dir.path())
        .args(["extract", "no_such_video.mp4"])
        .assert()
        .failure();
}

#[test]
fn batch_on_non_directory_fails() {
    // Fails regardless of FFmpeg availability, same reasoning as above.
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("not_a_dir.txt");
    fs::write(&file_path, b"hello").unwrap();

    bin()
        .args(["batch", file_path.to_str().unwrap()])
        .assert()
        .failure();
}

/// Generates a tiny 1-second sample video (silent-ish sine tone + test pattern)
/// using FFmpeg's `lavfi` synthetic sources, so the test does not depend on any
/// external media file.
fn generate_sample_video(dir: &std::path::Path) -> std::path::PathBuf {
    let output = dir.join("sample.mp4");
    let status = StdCommand::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=1:size=64x64:rate=10",
            "-f",
            "lavfi",
            "-i",
            "sine=duration=1",
            "-shortest",
        ])
        .arg(&output)
        .output()
        .expect("failed to run ffmpeg to generate sample video");

    assert!(
        status.status.success(),
        "ffmpeg failed to generate sample video: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    output
}

#[test]
fn extract_produces_non_empty_output_file() {
    if !ffmpeg_available() {
        eprintln!(
            "Nota: FFmpeg no está disponible, se omite extract_produces_non_empty_output_file"
        );
        return;
    }

    let dir = tempfile::tempdir().unwrap();
    let video = generate_sample_video(dir.path());
    let video_name = video.file_name().unwrap().to_str().unwrap().to_string();

    bin()
        .current_dir(dir.path())
        .args(["extract", &video_name, "-f", "wav"])
        .assert()
        .success();

    let output_path = dir.path().join("sample.wav");
    assert!(output_path.exists(), "expected output file to exist");
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0, "expected output file to be non-empty");
}

#[test]
fn batch_produces_non_empty_output_file() {
    if !ffmpeg_available() {
        eprintln!("Nota: FFmpeg no está disponible, se omite batch_produces_non_empty_output_file");
        return;
    }

    let dir = tempfile::tempdir().unwrap();
    generate_sample_video(dir.path());

    bin()
        .current_dir(dir.path())
        .args(["batch", ".", "-f", "wav"])
        .assert()
        .success();

    let output_path = dir.path().join("sample.wav");
    assert!(output_path.exists(), "expected output file to exist");
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0, "expected output file to be non-empty");
}
