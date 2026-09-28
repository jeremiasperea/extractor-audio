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

El ejecutable se generará en `target/release/audio_extractor`

## Uso

### Extraer audio de un solo video

```bash
# Con formato MP3 (predeterminado)
./target/release/audio_extractor extract video.mp4

# Especificando formato
./target/release/audio_extractor extract video.mp4 wav
./target/release/audio_extractor extract video.mkv flac
```

### Extracción por lotes (directorio completo)

```bash
# Procesar todos los videos en un directorio
./target/release/audio_extractor batch ./mis_videos

# Con formato específico
./target/release/audio_extractor batch ./mis_videos wav
```

### Ver ayuda

```bash
./target/release/audio_extractor help
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
./target/release/audio_extractor extract pelicula.mp4

# Extraer en formato WAV sin pérdida
./target/release/audio_extractor extract video.mkv wav

# Procesar todos los videos de una carpeta a FLAC
./target/release/audio_extractor batch ./descargas flac
```

## Notas

- Los archivos de audio se generan en el mismo directorio que el video fuente
- La calidad predeterminada es 192kbps para formatos con pérdida (MP3, AAC)
- Los formatos sin pérdida (WAV, FLAC) mantienen la calidad original
- El programa verifica la existencia de FFmpeg antes de ejecutar

## Licencia

MIT
