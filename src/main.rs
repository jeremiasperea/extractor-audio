use std::env;
use std::path::Path;

use clap::ValueEnum;
use extractor_audio::error::ExtractError;
use extractor_audio::extractor;
use extractor_audio::format::AudioFormat;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "extract" => {
            if args.len() < 3 {
                println!("Error: Debes especificar el archivo de video");
                print_usage();
                return;
            }
            let video_path = &args[2];
            let format_arg = if args.len() >= 4 { &args[3] } else { "mp3" };
            match parse_format(format_arg) {
                Ok(format) => run_extract(video_path, format),
                Err(message) => println!("{}", message),
            }
        }
        "batch" => {
            if args.len() < 3 {
                println!("Error: Debes especificar el directorio");
                print_usage();
                return;
            }
            let directory = &args[2];
            let format_arg = if args.len() >= 4 { &args[3] } else { "mp3" };
            match parse_format(format_arg) {
                Ok(format) => run_batch(directory, format),
                Err(message) => println!("{}", message),
            }
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            println!("Comando desconocido: {}", command);
            print_usage();
        }
    }
}

fn parse_format(format_arg: &str) -> Result<AudioFormat, String> {
    AudioFormat::from_str(format_arg, true)
        .map_err(|_| format!("Formato no soportado: {}", format_arg))
}

fn print_usage() {
    println!("\nExtractor de Audio de Videos");
    println!("=============================\n");
    println!("Uso:");
    println!("  audio_extractor extract <video> [formato]");
    println!("  audio_extractor batch <directorio> [formato]\n");
    println!("Comandos:");
    println!("  extract    Extrae audio de un solo video");
    println!("  batch      Extrae audio de todos los videos en un directorio");
    println!("  help       Muestra esta ayuda\n");
    println!("Formatos soportados:");
    println!("  mp3 (predeterminado), wav, aac, flac, ogg, m4a\n");
    println!("Ejemplos:");
    println!("  audio_extractor extract video.mp4");
    println!("  audio_extractor extract video.mkv wav");
    println!("  audio_extractor batch ./videos mp3");
}

fn run_extract(video_path: &str, format: AudioFormat) {
    println!("Extrayendo audio de: {}", video_path);
    println!("Formato de salida: {}", format.extension());

    match extractor::extract_audio(Path::new(video_path), format) {
        Ok(output) => {
            println!("Extraccion completada exitosamente: {}", output.display());
        }
        Err(err) => report_error(&err),
    }
}

fn run_batch(directory: &str, format: AudioFormat) {
    let videos = match extractor::find_videos(Path::new(directory)) {
        Ok(videos) => videos,
        Err(err) => {
            report_error(&err);
            return;
        }
    };

    if videos.is_empty() {
        println!("No se encontraron archivos de video en el directorio");
        return;
    }

    for (index, video) in videos.iter().enumerate() {
        println!("\n[{}/{}] Procesando archivo:", index + 1, videos.len());
        run_extract(&video.display().to_string(), format);
    }

    println!(
        "\nProceso completado. {} archivo(s) procesado(s)",
        videos.len()
    );
}

fn report_error(err: &ExtractError) {
    println!("Error: {}", err);
}
