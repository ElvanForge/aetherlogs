# AetherLogs DevLog

## Project Overview
High-performance log analysis tool. Go orchestrates the API and DB; Rust handles intensive string parsing.

## [2026-01-28] - Workspace Initialization
- **Action:** Bootstrapped `cmd/internal` Go structure and Rust crate.
- **Decision:** Chose `os/exec` for the Go-to-Rust bridge. 
- **Rationale:** Avoids the complexity of CGO and provides process-level isolation. If the parser crashes, the API stays up.
- **Ops:** Created a `Makefile` to standardize builds across Fedora and Pop!_OS.

## [2026-02-26]- Technical Achievements for Today's Devlog:

    Memory Mapping over Parsing: Implemented a "Zero-Copy" strategy. Instead of looping through bytes to find data, we mapped a raw byte stream directly to C-style structs using #[repr(C, packed)].

    The 16-Byte Header Mystery: Deciphered why a header that "looks" like 15 bytes of data is actually 16 bytes in memory (handling the unused padding byte and alignment).

    Struct Alignment Safety: Encountered and solved the E0793 error. Learned that Rust's compiler protects the CPU from "unaligned references" in packed structs by requiring us to copy values to the stack before use.

    The Binary Chain: Successfully mapped the sequential flow of an EVTC file:

        Header (16 bytes)

        Agent Count (4 bytes)

        Agent Array (N * 96 bytes)

    Mocking for Integrity: Built a manual binary buffer in a test harness to verify the parser logic without needing a massive external data source.

Project State:

Current Status: parser-core successfully identifies Boss IDs, Agent Counts, and Raw Addresses from binary streams.
Next Milestone: Converting Null-Terminated byte arrays into UTF-8 strings and bridging the Rust result back to the Go CLI.