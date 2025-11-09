//! Placeholder for experiment workflow integration tests.
//!
//! Verifies that `cargo run --package tiny-agent-trainer -- --config ...`
//! correctly parses the config, runs, and generates the expected
//! output files in the `results/` directory.
//!
//! These tests will use a special "smoke test" config with
//! `epochs: 1` and a tiny dataset.