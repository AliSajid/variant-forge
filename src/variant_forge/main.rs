// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Module for the Server
//!
//! Filler
//!
//! TODO: Add more detailed docs
mod cli;

use clap::Parser;
use cli::Cli;
use tracing_subscriber::FmtSubscriber;
use variant_forge_lib::utils::create_router;

/// Main function for the server
/// TODO: Add more detailed docs
#[tokio::main]
#[allow(clippy::missing_panics_doc)]
pub async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Failed to set subscriber");
        std::process::exit(3);
    }

    let cli = Cli::parse();

    let bind_addr = format!("{}:{}", cli.bind, cli.port);

    let router = create_router();

    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(listener) => {
            println!("Server started at http://{}", &bind_addr);
            listener
        }
        Err(e) => {
            eprintln!("Failed to start server: {e}");
            std::process::exit(1);
        }
    };

    match axum::serve(listener, router).await {
        Ok(()) => {
            println!("Listening on: http://{}", &bind_addr);
        }
        Err(e) => {
            eprintln!("Server error: {e}");
            std::process::exit(2);
        }
    }
}
