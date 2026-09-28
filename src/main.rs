use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use extractor_audio::error::ExtractError;
use extractor_audio::extractor;
use extractor_audio::format::AudioFormat;

/// Extractor de audio de videos usando FFmpeg.
#[derive(Parser, Debug)]
#[command(
    name = "extractor_audio",
    version,
    about = "Extrae el audio de archivos de video usando FFmpeg.",
    after_help = "EJEMPLOS:\n  extractor_audio extract video.mp4\n  extractor_audio extract video.mkv otro.avi -f wav\n  extractor_audio batch ./videos -f flac"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Extrae audio de uno o más archivos de video
    Extract {
        /// Archivo(s) de video de entrada
        #[arg(required = true)]
        videos: Vec<PathBuf>,

        /// Formato de audio de salida
        #[arg(short = 'f', long, value_enum, default_value = "mp3")]
        format: AudioFormat,
    },
    /// Extrae audio de todos los videos encontrados en un directorio
    Batch {
        /// Directorio que contiene los videos a procesar
        directory: PathBuf,

        /// Formato de audio de salida
        #[arg(short = 'f', long, value_enum, default_value = "mp3")]
        format: AudioFormat,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Err(err) = extractor::ensure_ffmpeg() {
        eprintln!("Error: {}", err);
        return ExitCode::FAILURE;
    }

    let (ok, failed) = match cli.command {
        Command::Extract { videos, format } => process_files(&videos, format),
        Command::Batch { directory, format } => match extractor::find_videos(&directory) {
            Ok(videos) if videos.is_empty() => {
                println!("No se encontraron archivos de video en el directorio");
                return ExitCode::FAILURE;
            }
            Ok(videos) => process_files(&videos, format),
            Err(err) => {
                eprintln!("Error: {}", err);
                return ExitCode::FAILURE;
            }
        },
    };

    println!("\nResumen: {} exitoso(s), {} fallido(s)", ok, failed);

    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Procesa cada archivo de video, continuando tras un fallo individual.
/// Devuelve el número de éxitos y de fallos.
fn process_files(videos: &[PathBuf], format: AudioFormat) -> (usize, usize) {
    let total = videos.len();
    let mut ok = 0usize;
    let mut failed = 0usize;

    for (index, video) in videos.iter().enumerate() {
        println!(
            "\n[{}/{}] Extrayendo audio de: {}",
            index + 1,
            total,
            video.display()
        );

        match extractor::extract_audio(video, format) {
            Ok(output) => {
                println!("Extraccion completada exitosamente: {}", output.display());
                ok += 1;
            }
            Err(err) => {
                report_error(&err);
                failed += 1;
            }
        }
    }

    (ok, failed)
}

fn report_error(err: &ExtractError) {
    eprintln!("Error: {}", err);
}
