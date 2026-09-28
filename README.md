# Extractor de Audio de Videos en Rust

Programa en Rust para extraer audio de archivos de video utilizando FFmpeg.

## Requisitos

1. Rust (versión 1.70 o superior)
2. FFmpeg instalado en el sistema

### Instalar FFmpeg

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
Descarga desde https://ffmpeg.org/download.html y añade al PATH

## Compilación

```bash
cargo build --release
```

El ejecutable se generará en `target/release/extractor_audio`

## Uso

> **Nota:** el formato ahora se indica con el flag `-f`/`--format` (antes era un argumento
> posicional). El binario se llama `extractor_audio`.

### Extraer audio de uno o más videos

```bash
# Con formato MP3 (predeterminado)
./target/release/extractor_audio extract video.mp4

# Especificando formato
./target/release/extractor_audio extract video.mkv -f wav
./target/release/extractor_audio extract video.mkv -f flac

# Múltiples archivos en una sola invocación
./target/release/extractor_audio extract video1.mp4 video2.mkv video3.mov -f wav
```

### Extracción por lotes (directorio completo)

```bash
# Procesar todos los videos en un directorio
./target/release/extractor_audio batch ./mis_videos

# Con formato específico
./target/release/extractor_audio batch ./mis_videos -f wav
```

### Ver ayuda

```bash
./target/release/extractor_audio --help
./target/release/extractor_audio extract --help
```

## Formatos soportados

- mp3 (predeterminado)
- wav
- aac
- flac
- ogg
- m4a

## Características

- Extracción de audio de un solo archivo
- Procesamiento por lotes de directorios completos
- Soporte para múltiples formatos de audio
- Soporte para formatos de video: mp4, avi, mkv, mov, flv, wmv, webm, m4v
- Calidad de audio configurable según formato
- Verificación de archivos y FFmpeg antes de procesar

## Ejemplos

```bash
# Extraer audio MP3 de alta calidad
./target/release/extractor_audio extract pelicula.mp4

# Extraer en formato WAV sin pérdida
./target/release/extractor_audio extract video.mkv -f wav

# Procesar todos los videos de una carpeta a FLAC
./target/release/extractor_audio batch ./descargas -f flac
```

## Notas

- Los archivos de audio se generan en el directorio desde el que se ejecuta el programa
  (directorio de trabajo actual), no necesariamente junto al video de origen
- Si algún archivo falla durante `extract`/`batch`, el programa continúa con el resto y finaliza
  con código de salida distinto de cero
- La calidad predeterminada es 192kbps para formatos con pérdida (MP3, AAC)
- Los formatos sin pérdida (WAV, FLAC) mantienen la calidad original
- El programa verifica la existencia de FFmpeg antes de ejecutar

## Licencia

MIT
