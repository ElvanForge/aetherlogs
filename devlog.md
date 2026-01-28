# AetherLogs DevLog

## Project Overview
High-performance log analysis tool. Go orchestrates the API and DB; Rust handles intensive string parsing.

## [2026-01-28] - Workspace Initialization
- **Action:** Bootstrapped `cmd/internal` Go structure and Rust crate.
- **Decision:** Chose `os/exec` for the Go-to-Rust bridge. 
- **Rationale:** Avoids the complexity of CGO and provides process-level isolation. If the parser crashes, the API stays up.
- **Ops:** Created a `Makefile` to standardize builds across Fedora and Pop!_OS.