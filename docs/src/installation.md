# Installation

There are several ways to install FindeRS. Choose the method that works best for you.

## From Pre-built Binaries (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/ydkadri/finders/releases):

```bash
# Linux (x86_64)
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-x86_64-linux.tar.gz

# macOS (Apple Silicon)
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-aarch64-macos.tar.gz

# macOS (Intel)
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-x86_64-macos.tar.gz

# Windows
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-x86_64-windows.zip
```

**Available architectures:**
- `x86_64-linux` - Linux (x86_64)
- `aarch64-macos` - macOS (Apple Silicon)
- `x86_64-macos` - macOS (Intel)
- `x86_64-windows` - Windows (use `.zip` instead of `.tar.gz`)

### Extract and Install

```bash
tar -xzf finder-<version>-<arch>.tar.gz  # or unzip for Windows
sudo mv finder /usr/local/bin/           # or add to PATH on Windows
```

### Verify Checksum (Optional)

```bash
wget https://github.com/ydkadri/finders/releases/latest/download/finder-<version>-<arch>.tar.gz.sha256
sha256sum -c finder-<version>-<arch>.tar.gz.sha256
```

## From Source (via Cargo)

If you have Rust installed:

```bash
cargo install finders
```

This compiles from source and installs the binary in `~/.cargo/bin/`. Make sure this directory is in your `PATH`.

## From Source (Manual Build)

```bash
# Clone the repository
git clone https://github.com/ydkadri/finders.git
cd finders

# Build in release mode
cargo build --release

# The binary will be at target/release/finder
sudo cp target/release/finder /usr/local/bin/
```

## Verify Installation

After installing, verify it works:

```bash
finder --version
```

You should see output like:
```
finder 3.0.0
```

## Updating

### Binary Installation

Download and install the latest release following the same steps above.

### Cargo Installation

```bash
cargo install finders --force
```

## Uninstalling

### Binary Installation

```bash
sudo rm /usr/local/bin/finder
```

### Cargo Installation

```bash
cargo uninstall finders
```

## Next Steps

- Follow the [Quick Start](./quick-start.md) guide to learn the basics
- Explore [common use cases](./examples/common-use-cases.md)
- Check the [CLI Reference](./cli-reference.md) for all available options
