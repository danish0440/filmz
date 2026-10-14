//! Golden-image tests for FilmZ's renderer (test-only crate; the library is empty).
//!
//! `tests/golden.rs` builds small procedural projects (no media files), renders one frame of each
//! through the CPU reference compositor (`filmz-render`) and compares it with a committed PNG
//! in `goldens/` using `filmz_testkit::golden`. The same scenes are composited on the GPU
//! (`filmz-gpu`) when an adapter is available and compared with the CPU result.
//!
//! Regenerate references with `FILMZ_BLESS=1 cargo test -p filmz-golden`.
