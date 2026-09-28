//! format.rs — Formatos de audio soportados y sus parámetros de FFmpeg.

use clap::ValueEnum;

/// Formato de audio de salida soportado.
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AudioFormat {
    #[default]
    Mp3,
    Wav,
    Aac,
    Flac,
    Ogg,
    M4a,
}

impl AudioFormat {
    /// Extensión de archivo (sin punto) para este formato.
    pub fn extension(&self) -> &'static str {
        match self {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Wav => "wav",
            AudioFormat::Aac => "aac",
            AudioFormat::Flac => "flac",
            AudioFormat::Ogg => "ogg",
            AudioFormat::M4a => "m4a",
        }
    }

    /// Codec de FFmpeg (`-acodec`) para este formato.
    pub fn codec(&self) -> &'static str {
        match self {
            AudioFormat::Mp3 => "libmp3lame",
            AudioFormat::Wav => "pcm_s16le",
            AudioFormat::Aac => "aac",
            AudioFormat::Flac => "flac",
            AudioFormat::Ogg => "libvorbis",
            AudioFormat::M4a => "aac",
        }
    }

    /// Argumentos adicionales de calidad para FFmpeg, si aplica.
    pub fn quality_args(&self) -> &'static [&'static str] {
        match self {
            AudioFormat::Mp3 | AudioFormat::Aac | AudioFormat::M4a => &["-b:a", "192k"],
            AudioFormat::Ogg => &["-q:a", "5"],
            AudioFormat::Wav | AudioFormat::Flac => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_format_is_mp3() {
        assert_eq!(AudioFormat::default(), AudioFormat::Mp3);
    }

    #[test]
    fn extensions_match_format() {
        assert_eq!(AudioFormat::Mp3.extension(), "mp3");
        assert_eq!(AudioFormat::Wav.extension(), "wav");
        assert_eq!(AudioFormat::Aac.extension(), "aac");
        assert_eq!(AudioFormat::Flac.extension(), "flac");
        assert_eq!(AudioFormat::Ogg.extension(), "ogg");
        assert_eq!(AudioFormat::M4a.extension(), "m4a");
    }

    #[test]
    fn codecs_match_format() {
        assert_eq!(AudioFormat::Mp3.codec(), "libmp3lame");
        assert_eq!(AudioFormat::Wav.codec(), "pcm_s16le");
        assert_eq!(AudioFormat::Aac.codec(), "aac");
        assert_eq!(AudioFormat::Flac.codec(), "flac");
        assert_eq!(AudioFormat::Ogg.codec(), "libvorbis");
        assert_eq!(AudioFormat::M4a.codec(), "aac");
    }

    #[test]
    fn quality_args_match_format() {
        assert_eq!(AudioFormat::Mp3.quality_args(), &["-b:a", "192k"]);
        assert_eq!(AudioFormat::Aac.quality_args(), &["-b:a", "192k"]);
        assert_eq!(AudioFormat::M4a.quality_args(), &["-b:a", "192k"]);
        assert_eq!(AudioFormat::Ogg.quality_args(), &["-q:a", "5"]);
        assert!(AudioFormat::Wav.quality_args().is_empty());
        assert!(AudioFormat::Flac.quality_args().is_empty());
    }
}
