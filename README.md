# Anvil

Anvil is a version-controlled artifact management tool that creates a blockchain-like chain of build artifacts for your projects. It allows you to pack, store, and install versioned binaries with full traceability back to their source commits.

## Features

- **Blockchain-style artifact tracking** - Each build artifact is stored as a "block" with cryptographic hashes linking to previous versions
- **Version management** - Tag and track multiple versions of your build artifacts
- **Git integration** - Automatically associates builds with git commits and can create git tags
- **Artifact deduplication** - Reuses existing blocks when artifacts haven't changed
- **Remote installation** - Install packages directly from git repositories
- **Chain validation** - Verify integrity of the entire artifact chain

## Installation

### From Source

Requires Rust toolchain (1.70+ recommended):

```bash
git clone https://github.com/pepedinho/anvil.git
cd anvil
cargo build --release
```

The binary will be available at `target/release/anvil`.

### Adding to PATH

To use Anvil globally, add the binary to your PATH:

```bash
# Copy to local bin
cp target/release/anvil ~/.local/bin/

# Or add Anvil's bin directory to PATH
echo 'export PATH="$HOME/.anvil/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Usage

### Initialize a Project

Create an `.anvil` directory in your project root with a configuration file:

```bash
mkdir -p .anvil
```

Create `.anvil/anvil.yml`:

```yaml
project:
  name: "my-project"

build:
  artifact_dir: "target/release"
  entrypoint: "target/release/my-project"
  command: "cargo build --release"
  incremental: true
  jit: false

env:
  RUST_LOG: "debug"
  APP_ENV: "production"
```

### Pack a Version

Build and store a new version of your artifact:

```bash
# Pack a version
anvil pack 0.1.0

# Pack a version and create a git tag
anvil pack 0.1.0 --tag
```

This will:
1. Run your build command
2. Compute a hash of the resulting artifact
3. Store the artifact in `~/.anvil/store/<project-name>/`
4. Add a new block to `.anvil/blocks.json`
5. Optionally create a git tag

### Install a Package

Install a package from a git repository:

```bash
# Install the latest version
anvil install https://github.com/user/project.git

# Install a specific version
anvil install https://github.com/user/project.git --version 0.2.0
```

This will:
1. Clone or update the repository to `~/.anvil/repo/<project-name>/`
2. Checkout the commit associated with the specified version
3. Build the project using its configuration
4. Install the binary to `~/.anvil/bin/<project-name>`

### Switch Versions

Switch between installed versions of a project:

```bash
anvil switch my-project 0.1.0
```

## Configuration

### `anvil.yml` Options

| Field | Description |
|-------|-------------|
| `project.name` | Name of the project |
| `build.artifact_dir` | Directory containing build artifacts |
| `build.entrypoint` | Path to the main binary/artifact |
| `build.command` | Shell command to build the project |
| `build.incremental` | Enable incremental builds |
| `build.jit` | Enable JIT compilation (if applicable) |
| `dependency_script` | Optional script to run before building |
| `env` | Environment variables for the build |

### `blocks.json` Format

The blocks file tracks all packed versions:

```json
[
  {
    "artefact_hash": "<sha256-hash>",
    "artefact_type": "Bin",
    "created_at": { "secs_since_epoch": 1234567890, "nanos_since_epoch": 0 },
    "version": "0.1.0",
    "git_commit": "<commit-sha>",
    "prev_block_hash": null,
    "block_hash": "<sha256-hash>",
    "entrypoint": "target/release/my-project"
  }
]
```

## Project Structure

```
my-project/
├── .anvil/
│   ├── anvil.yml      # Project configuration
│   └── blocks.json    # Artifact chain (version history)
├── src/
│   └── ...
└── ...
```

Global Anvil directories:

```
~/.anvil/
├── store/             # Artifact storage (by project)
│   └── <project>/
├── repo/              # Cloned repositories
│   └── <project>/
├── bin/               # Installed binaries
│   └── <project>
└── meta/              # Installation metadata
    └── <project>.json
```

## How It Works

### Artifact Chain

Anvil uses a blockchain-inspired approach to track build artifacts:

1. **Genesis Block** - The first version packed for a project
2. **Subsequent Blocks** - Each new version references the previous block's hash
3. **Integrity Verification** - The chain can be validated to detect tampering

### Deduplication

When packing a new version, Anvil computes the SHA-256 hash of the artifact. If an identical artifact already exists in the chain, the existing block is reused instead of creating a duplicate.

### Version Resolution

When installing a package:
- If no version is specified, the latest (last block) is used
- If a version is specified, Anvil finds the block with the matching version string

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## License

This project is open source. See the repository for license details.
