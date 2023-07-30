# rscp

A :rocket: blazingly fast Rust-based file transfer utility for quick sharing in local network.

## Features

- **Blazingly Fast Transfers:** Leverage the power of Rust's high-performance capabilities for lightning-fast file uploads and downloads.

- **QR Code Convenience:** Users can simply scan QR code to access the file download/upload portal, eliminating the need for complex URLs or manual input

- **User-Friendly Web Interface:** Intuitive web interface for users to easily upload and download files without technical complexities.

- **Progress Tracking:** Real-time progress tracking during file uploads, providing users with visibility into the transfer status.

- **Support for Large Files:** Seamlessly transfer large files without sacrificing speed or stability.

- **Cross-Platform Compatibility:** Compatibility with major operating systems (Windows, Linux).

- **Easy Installation and Usage:** Get started quickly with straightforward installation instructions enabling users to setup the utility effortlessly

## Usage

### Send

https://github.com/VulnX/rscp/assets/62636727/a4b6b95b-c228-4bc3-9726-afadbc84f805

### Receive

https://github.com/VulnX/rscp/assets/62636727/275b3125-7e54-4f5d-b03d-a459afbd0528

## Installation

### From source

Make sure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Get the source code by cloning this project:

```bash
git clone https://github.com/vulnx/rscp.git
```

You **need** to set this project as "nightly" to build a release for yourself:

```bash
rustup override set nightly
```

Then build the utility in release mode:

```bash
cargo build --release
```

Once the build process is complete, you can find the executable in the `target/release/` directory.

### Linux

Download the latest linux .tar.gz archive from the [Releases](https://github.com/VulnX/rscp/releases) page, extract it, set execution permissions, then move the binary to the local bin directory.

```bash
# Extract the archive
tar xf rscp_1.1.0_linux_x86_64.tar.gz
# Set execution permissions
sudo chmod +x rscp
# Move the binary to bin directory
sudo mv rscp /usr/local/bin
```

Now you simply run the application as:

```bash
rscp
```

### Windows

Download the latest windows .tar.gz archive from the [Releases](https://github.com/VulnX/rscp/releases) page, extract it, and run the executable.

> NOTE: You might need to allow it through firewall if the port 8000 is not open

## Additional

This is mainly a personal project for me to learn Rust and Rocket by making an actual usable program.

The inspiration is from [qrcp](https://github.com/claudiodangelis/qrcp)
