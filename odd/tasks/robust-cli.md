# Feature: robust-cli

## Objective
Incorporate the useful patterns from `jeremiasperea/audio-transcriber` into this audio extractor:
typed errors with `Result` propagation, a `clap` derive CLI with multi-file input, a module split,
and tests.

## Problem / Why
`src/main.rs` is a single 198-line file with hand-rolled `env::args()` parsing, errors reported via
`println!` + `return` (exit code is always 0), a panic risk in batch mode
(`path.to_str().unwrap()`, non-UTF-8 paths), format validated only after side-effect messages, and
zero tests.

## Scope
- In: module split (lib + bin), `thiserror` error type + `anyhow` at the edge, `clap` derive CLI,
  multiple inputs for `extract`, non-zero exit code on failures, unit + integration tests,
  README usage update.
- Out: colored output / progress spinner, Dockerfile, transcription pipeline, changing where output
  files are written (currently the current working directory — kept as-is).

## Constraints
- User-facing CLI messages stay in Spanish (existing project language); identifiers/comments in English.
- Keep supported formats and FFmpeg codec/quality flags identical.
- Planning heuristic: ~400 authored changed lines per task (advisory only).

## Configuration
- TDD: off (source: no project/session TDD configuration). Runner: `cargo test`.
- RDD: on (source: global). Assess each work-unit commit.
- Delivery strategy: ask-on-risk (default). Forecast: ~450-550 authored changed lines total.

## Tasks
- [x] T1 refactor: split into `src/lib.rs` + `error.rs` + `format.rs` + `extractor.rs`; `AudioFormat`
      enum; `ExtractError` (thiserror); all fallible fns return `Result`; remove `unwrap`; unit tests
      for pure helpers. Route: delegated (writer trigger: 4+ non-trivial files).
- [x] T2 feat: `clap` derive CLI (`extract <videos...> -f`, `batch <dir> -f`), `anyhow` in `main`,
      per-file failure summary, non-zero exit on any failure; README usage update.
      Route: delegated (same writer, multi-file).
- [ ] T3 test: integration tests in `tests/` — end-to-end extraction on an FFmpeg-generated sample
      (skipped if FFmpeg is missing) and CLI behavior (help, invalid format, missing file).
      Route: delegated (same writer).

## Acceptance criteria
- `cargo build`, `cargo clippy --all-targets -- -D warnings`, `cargo test` pass.
- `extractor_audio --help`, `extract a.mp4 b.mkv -f wav`, `batch ./dir -f mp3` work.
- Invalid format rejected by clap before any work; any failed file → exit code 1.

## Progress / Evidence

### T1 — `refactor: split into modules with typed errors`
- Added `src/lib.rs` exposing `error`, `format`, `extractor`; `src/main.rs` is now the binary using
  `extractor_audio::...`.
- `format.rs`: `AudioFormat` (derives `clap::ValueEnum`, `Default` = `Mp3`) with `extension()`,
  `codec()`, `quality_args()` — identical mapping to the previous inline `match` arms.
- `error.rs`: `ExtractError` (`thiserror`) with `InputNotFound`, `NotADirectory`,
  `InvalidFileName`, `FfmpegNotFound`, `FfmpegFailed { stderr }`, `Io(#[from] std::io::Error)`;
  all Display messages in Spanish.
- `extractor.rs`: pure/fallible helpers `output_path`, `build_ffmpeg_args`, `is_video_file`,
  `find_videos` (sorted via `Vec::sort`), `ensure_ffmpeg`, `extract_audio`. No `unwrap`/`expect`
  outside `#[cfg(test)]`; `to_str().unwrap()` on batch paths removed (now uses `Path`/`display()`).
- `main.rs` (T1 scope only) still does manual `env::args()` parsing but calls the new
  `Result`-based lib API (`extractor::extract_audio`, `extractor::find_videos`) and formats errors
  via `ExtractError`'s `Display`. Exit-code fix and clap CLI are deliberately deferred to T2, per
  the task instructions ("main.rs may still use the old manual arg parsing in this commit").
- Added `clap = { version = "4.5", features = ["derive"] }` and `thiserror = "1.0"` to
  `[dependencies]`, `tempfile = "3.10"` to `[dev-dependencies]` (used by `find_videos` tests).
- Decision: `output_path` returns a path relative to the current working directory (just
  `<stem>.<ext>`), matching the previous behavior of writing output next to the *invocation* cwd,
  not next to the input file (per Scope/Out: "not changing where output files are written").

Verification (from repo root):
- `cargo fmt`: applied, no functional diff (only formatting).
- `cargo build`: OK — `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 58.35s`.
- `cargo clippy --all-targets -- -D warnings`: OK — `Finished \`dev\` profile ... in 3.84s`, no
  warnings.
- `cargo test`: OK — `test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
  (lib unit tests for `format` and `extractor`); 0 tests in `main.rs`/doctests (expected at this
  stage).

Commit: recorded below after commit (see end of file).

### T2 — `feat: clap-based CLI with multi-file extraction`
- `main.rs` rewritten with `clap` derive: `Cli { command: Command }`, `Command::Extract { videos:
  Vec<PathBuf> (required, 1+), format: AudioFormat }` and `Command::Batch { directory: PathBuf,
  format: AudioFormat }`. `-f`/`--format` uses `value_enum` with `default_value = "mp3"`, so an
  invalid format (`-f xyz`) is rejected by clap with exit code 2 before any file is touched or
  FFmpeg is invoked (verified manually).
- Decision: `main() -> std::process::ExitCode` (not `anyhow::Result`) — chosen because there is no
  single top-level fallible operation to propagate with `?`; the function explicitly aggregates
  per-file outcomes and picks the exit code itself, which `ExitCode` expresses directly without an
  `anyhow` dependency.
- FFmpeg presence (`extractor::ensure_ffmpeg()`) is checked exactly once up front, before any file
  is processed, per task instructions.
- Per-file processing (`process_files`) prints `[i/N] Extrayendo audio de: <path>` (N is the total
  file count, known up front for both `extract` and `batch`) and continues after a per-file
  failure; a final `Resumen: X exitoso(s), Y fallido(s)` line is always printed. Errors go to
  stderr via `eprintln!` (previously `println!`, which also fixes errors not being visible when
  stdout is piped separately from stderr).
- Decision (no-videos case): `batch` on a directory with zero recognized video files prints
  "No se encontraron archivos de video en el directorio" and returns `ExitCode::FAILURE` (exit 1).
  Rationale: matches the acceptance criterion ("no videos were found in batch mode" → treated as a
  failure condition, consistent with "any failed file → exit code 1"), and gives scripts a
  reliable non-zero signal that nothing was produced instead of silently exiting 0.
- Exit codes: 0 only if every requested/found file succeeded; 1 if FFmpeg is missing, the batch
  directory is invalid/empty, or at least one file failed; clap's own arg-parsing errors (e.g.
  invalid `-f` value, missing required `<VIDEOS>`) use clap's own exit code (2), which is
  clap's standard behavior and out of scope to override.
- README `## Uso` rewritten for the new flag-based syntax (`extract`/`batch` with `-f`), including
  a multi-file example and a note about the breaking change (format used to be a positional arg).
  Also corrected two pre-existing inaccuracies noticed while editing the same section: the binary
  name in "Compilación" (`audio_extractor` → `extractor_audio`, matching `Cargo.toml`'s package
  name) and the "Notas" claim that output always lands next to the source video (it lands in the
  current working directory — unchanged behavior, now documented correctly per this feature's
  explicit Scope/Out).

Verification (from repo root):
- `cargo fmt`: applied, no functional diff (only formatting).
- `cargo build`: OK — `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 0.49s`.
- `cargo clippy --all-targets -- -D warnings`: OK — no warnings.
- `cargo test`: OK — `test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
  (unchanged; T2 added no new unit tests, integration/CLI tests are T3's scope).
- Manual smoke test (not part of `cargo test`, informational only):
  - `extractor_audio --help` → prints usage, exit 0.
  - `extractor_audio extract -f xyz nope.mp4` → clap rejects before doing any work, exit 2.
  - `extractor_audio extract nope.mp4` → prints "El archivo 'nope.mp4' no existe" +
    "Resumen: 0 exitoso(s), 1 fallido(s)", exit 1.
  - `extractor_audio batch /nonexistent-dir` → prints "El archivo '/nonexistent-dir' no existe",
    exit 1.

## Next step
T3.

## Commit log
- T1: `faee960` — refactor: split into modules with typed errors
- T2: recorded in the T3 commit (see below)
