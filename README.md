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

https://github.com/VulnX/rscp/assets/62636727/b3d66901-eca0-493c-a564-fd5facfc33f6

### Receive

https://github.com/VulnX/rscp/assets/62636727/9e335534-69e1-4730-a24d-a9d146bb7c78

## Installation

### From source

Make sure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Get the source code by cloning this project:

```bash
git clone https://github.com/vulnx/rscp.git
```

Then build in release mode:

```bash
cargo build --release
```

Once the build process is complete, you can find the executable in the `target/release/` directory.

### Linux

Download the latest linux executable from the [Releases](https://github.com/VulnX/rscp/releases) page, set execution permissions, then move the binary to the local bin directory.

```bash
# Set execution permissions
sudo chmod +x rscp*
# Move the binary to bin directory
sudo mv rscp* /usr/local/bin/rscp
```

Now run the application:

```bash
rscp
```

### Windows

Download the latest windows executable from the [Releases](https://github.com/VulnX/rscp/releases) page, and run it.

> NOTE: You might need to allow it through the firewall.

## TODO
- Improve code documentation
- Add support for zipping files
- Add support for Android and Mac OS

## Additional

This is mainly a personal project for me to learn Rust by making an actual usable program.

The inspiration is from [qrcp](https://github.com/claudiodangelis/qrcp)
