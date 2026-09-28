//! extractor.rs — Lógica de extracción de audio con FFmpeg.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{ExtractError, Result};
use crate::format::AudioFormat;

/// Extensiones de video reconocidas (comparación case-insensitive).
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm", "m4v"];

/// Calcula la ruta de salida (relativa al directorio de trabajo actual) para
/// un archivo de entrada y un formato dados: mismo "stem" que el archivo de
/// entrada, con la extensión del formato de salida.
pub fn output_path(input: &Path, format: AudioFormat) -> Result<PathBuf> {
    let stem = input
        .file_stem()
        .ok_or_else(|| ExtractError::InvalidFileName(input.to_path_buf()))?;
    let mut output = PathBuf::from(stem);
    output.set_extension(format.extension());
    Ok(output)
}

/// Construye los argumentos de línea de comandos para invocar FFmpeg.
pub fn build_ffmpeg_args(input: &Path, output: &Path, format: AudioFormat) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec![
        OsString::from("-i"),
        input.as_os_str().to_os_string(),
        OsString::from("-vn"),
        OsString::from("-acodec"),
        OsString::from(format.codec()),
    ];
    for arg in format.quality_args() {
        args.push(OsString::from(arg));
    }
    args.push(output.as_os_str().to_os_string());
    args
}

/// Indica si una ruta tiene una extensión de video reconocida (case-insensitive).
pub fn is_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Busca archivos de video dentro de un directorio, de forma no recursiva.
/// Devuelve las rutas ordenadas para un resultado determinista.
pub fn find_videos(dir: &Path) -> Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Err(ExtractError::InputNotFound(dir.to_path_buf()));
    }
    if !dir.is_dir() {
        return Err(ExtractError::NotADirectory(dir.to_path_buf()));
    }

    let mut videos: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_video_file(path))
        .collect();

    videos.sort();
    Ok(videos)
}

/// Verifica que FFmpeg esté instalado y accesible en el PATH.
pub fn ensure_ffmpeg() -> Result<()> {
    let found = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if found {
        Ok(())
    } else {
        Err(ExtractError::FfmpegNotFound)
    }
}

/// Extrae el audio de `input` al formato indicado, escribiendo el resultado
/// en el directorio de trabajo actual. Devuelve la ruta del archivo generado.
pub fn extract_audio(input: &Path, format: AudioFormat) -> Result<PathBuf> {
    if !input.exists() {
        return Err(ExtractError::InputNotFound(input.to_path_buf()));
    }

    let output = output_path(input, format)?;
    let args = build_ffmpeg_args(input, &output, format);

    let result = Command::new("ffmpeg")
        .args(&args)
        .output()
        .map_err(|_| ExtractError::FfmpegNotFound)?;

    if result.status.success() {
        Ok(output)
    } else {
        Err(ExtractError::FfmpegFailed {
            stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn output_path_uses_stem_and_format_extension() {
        let input = Path::new("videos/pelicula.mp4");
        let output = output_path(input, AudioFormat::Wav).unwrap();
        assert_eq!(output, PathBuf::from("pelicula.wav"));
    }

    #[test]
    fn output_path_fails_without_file_name() {
        let input = Path::new("..");
        assert!(output_path(input, AudioFormat::Mp3).is_err());
    }

    #[test]
    fn is_video_file_recognizes_known_extensions() {
        assert!(is_video_file(Path::new("clip.mp4")));
        assert!(is_video_file(Path::new("clip.MKV")));
        assert!(is_video_file(Path::new("clip.WebM")));
        assert!(!is_video_file(Path::new("audio.mp3")));
        assert!(!is_video_file(Path::new("no_extension")));
    }

    #[test]
    fn build_ffmpeg_args_contains_expected_flags() {
        let input = Path::new("in.mp4");
        let output = Path::new("out.mp3");
        let args = build_ffmpeg_args(input, output, AudioFormat::Mp3);
        let args: Vec<String> = args
            .into_iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            args,
            vec![
                "-i",
                "in.mp4",
                "-vn",
                "-acodec",
                "libmp3lame",
                "-b:a",
                "192k",
                "out.mp3"
            ]
        );
    }

    #[test]
    fn build_ffmpeg_args_for_wav_has_no_quality_flags() {
        let input = Path::new("in.mp4");
        let output = Path::new("out.wav");
        let args = build_ffmpeg_args(input, output, AudioFormat::Wav);
        let args: Vec<String> = args
            .into_iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            args,
            vec!["-i", "in.mp4", "-vn", "-acodec", "pcm_s16le", "out.wav"]
        );
    }

    #[test]
    fn find_videos_returns_sorted_video_files_only() {
        let dir = tempfile::tempdir().unwrap();
        for name in ["b.mp4", "a.mkv", "notes.txt", "c.MP4"] {
            File::create(dir.path().join(name)).unwrap();
        }

        let videos = find_videos(dir.path()).unwrap();
        let names: Vec<String> = videos
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        assert_eq!(names, vec!["a.mkv", "b.mp4", "c.MP4"]);
    }

    #[test]
    fn find_videos_fails_for_missing_directory() {
        let missing = Path::new("this/does/not/exist");
        assert!(matches!(
            find_videos(missing),
            Err(ExtractError::InputNotFound(_))
        ));
    }

    #[test]
    fn find_videos_fails_for_non_directory() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        File::create(&file_path).unwrap();
        assert!(matches!(
            find_videos(&file_path),
            Err(ExtractError::NotADirectory(_))
        ));
    }
}
