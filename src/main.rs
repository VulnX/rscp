///
///         rscp
///         A blazingly fast Rust-based file transfer utility for quick sharing in local network.
///
///         Author  : VulnX
///         Version : 2.0.0
///         License : MIT
///         GitHub  : https://github.com/vulnx/rscp
///


mod cli;
mod recv;
mod send;
mod server;
mod shutdown;
mod qr;

#[tokio::main]
async fn main() {
    cli::start().await;
}
