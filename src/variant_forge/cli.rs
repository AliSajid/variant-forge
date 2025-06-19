// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Command-line interface definition for the Variant Forge server.
//!
//! This module defines the CLI arguments used to configure the `aaprop-server` binary.
//! It uses [`clap`](https://docs.rs/clap) for parsing arguments from the command line or environment variables.
//!
//! The server supports configuration for network binding, port, and logging verbosity.
//!
//! # Environment Variables
//!
//! - `BIND_ADDRESS`: Overrides the default listen address (`127.0.0.1`)
//! - `BIND_PORT`: Overrides the default listen port (`8080`)
//! - `LOG_LEVEL`: Sets the verbosity level of logging (`info`, `debug`, etc.)
//!
//! # Example
//!
//! ```sh
//! aaprop-server --bind 0.0.0.0 --port 3000 --log debug
//! ```

use clap::{
    Parser,
    ValueEnum,
};

/// Command-line arguments for the `aaprop-server` binary.
#[derive(Parser, Debug)]
#[command(name = "aaprop-server", version, about)]
pub struct Cli {
    /// Network address the server will bind to.
    ///
    /// Can also be set using the `BIND_ADDRESS` environment variable.
    #[arg(
        short,
        long,
        default_value = "127.0.0.1",
        env = "BIND_ADDRESS",
        value_name = "LISTEN_ADDRESS"
    )]
    pub bind: String,

    /// Port number the server will listen on.
    ///
    /// Can also be set using the `BIND_PORT` environment variable.
    #[arg(
        short,
        long,
        default_value = "8080",
        env = "BIND_PORT",
        value_name = "LISTEN_PORT"
    )]
    pub port: u16,

    /// Logging verbosity level.
    ///
    /// Acceptable values: `trace`, `debug`, `info`, `warn`, `error`.
    /// Can also be set using the `LOG_LEVEL` environment variable.
    #[arg(
        short,
        long,
        default_value = "info",
        env = "LOG_LEVEL",
        value_name = "LOG_LEVEL"
    )]
    pub log: LogLevel,
}

/// Supported logging verbosity levels.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Very fine-grained logs, useful for debugging internals.
    Trace,

    /// Debug-level logs for development use.
    Debug,

    /// Informational messages (default).
    Info,

    /// Warnings that may indicate a problem.
    Warn,

    /// Errors that prevent normal operation.
    Error,
}
