//! error.rs — Tipos de error del programa

use std::path::PathBuf;
use thiserror::Error;

/// Errores producidos durante la extracción de audio.
#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("El archivo '{0}' no existe")]
    InputNotFound(PathBuf),

    #[error("'{0}' no es un directorio")]
    NotADirectory(PathBuf),

    #[error("No se pudo determinar el nombre del archivo para '{0}'")]
    InvalidFileName(PathBuf),

    #[error("FFmpeg no está instalado o no se encuentra en el PATH. Instálalo desde: https://ffmpeg.org/download.html")]
    FfmpegNotFound,

    #[error("Error durante la extracción con FFmpeg: {stderr}")]
    FfmpegFailed { stderr: String },

    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ExtractError>;
