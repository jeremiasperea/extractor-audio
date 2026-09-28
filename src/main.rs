use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;

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
            let output_format = if args.len() >= 4 { &args[3] } else { "mp3" };
            extract_audio(video_path, output_format);
        }
        "batch" => {
            if args.len() < 3 {
                println!("Error: Debes especificar el directorio");
                print_usage();
                return;
            }
            let directory = &args[2];
            let output_format = if args.len() >= 4 { &args[3] } else { "mp3" };
            batch_extract(directory, output_format);
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

fn extract_audio(video_path: &str, format: &str) {
    // Verificar que el archivo existe
    if !Path::new(video_path).exists() {
        println!("Error: El archivo '{}' no existe", video_path);
        return;
    }

    // Verificar que FFmpeg está instalado
    if !check_ffmpeg() {
        println!("Error: FFmpeg no está instalado o no se encuentra en el PATH");
        println!("Instala FFmpeg desde: https://ffmpeg.org/download.html");
        return;
    }

    // Generar nombre del archivo de salida
    let video_path_obj = Path::new(video_path);
    let output_name = match video_path_obj.file_stem() {
        Some(name) => format!("{}.{}", name.to_string_lossy(), format),
        None => {
            println!("Error: No se pudo determinar el nombre del archivo");
            return;
        }
    };

    println!("Extrayendo audio de: {}", video_path);
    println!("Formato de salida: {}", format);
    println!("Archivo de destino: {}", output_name);

    // Configurar parámetros según el formato
    let codec = match format {
        "mp3" => "libmp3lame",
        "wav" => "pcm_s16le",
        "aac" => "aac",
        "flac" => "flac",
        "ogg" => "libvorbis",
        "m4a" => "aac",
        _ => {
            println!("Formato no soportado: {}", format);
            return;
        }
    };

    // Ejecutar FFmpeg
    let mut command = Command::new("ffmpeg");
    command.arg("-i")
           .arg(video_path)
           .arg("-vn")  // Sin video
           .arg("-acodec")
           .arg(codec);

    // Agregar calidad según el formato
    match format {
        "mp3" => {
            command.arg("-b:a").arg("192k");
        }
        "aac" | "m4a" => {
            command.arg("-b:a").arg("192k");
        }
        "ogg" => {
            command.arg("-q:a").arg("5");
        }
        _ => {}
    }

    command.arg(&output_name);

    // Ejecutar el comando
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                println!("Extraccion completada exitosamente: {}", output_name);
            } else {
                println!("Error durante la extraccion:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("Error al ejecutar FFmpeg: {}", e);
        }
    }
}

fn batch_extract(directory: &str, format: &str) {
    let dir_path = Path::new(directory);

    if !dir_path.exists() {
        println!("Error: El directorio '{}' no existe", directory);
        return;
    }

    if !dir_path.is_dir() {
        println!("Error: '{}' no es un directorio", directory);
        return;
    }

    // Extensiones de video comunes
    let video_extensions = vec!["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm", "m4v"];

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let mut count = 0;
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            if video_extensions.contains(&ext_str.as_str()) {
                                count += 1;
                                println!("\n[{}/...] Procesando archivo:", count);
                                extract_audio(path.to_str().unwrap(), format);
                            }
                        }
                    }
                }
            }
            if count == 0 {
                println!("No se encontraron archivos de video en el directorio");
            } else {
                println!("\nProceso completado. {} archivo(s) procesado(s)", count);
            }
        }
        Err(e) => {
            println!("Error al leer el directorio: {}", e);
        }
    }
}

fn check_ffmpeg() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .is_ok()
}