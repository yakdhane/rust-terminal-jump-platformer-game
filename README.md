Sure, here's the beautifully formatted README.md file:

```
<p align="center">
  <img src="https://img.shields.io/badge/language-rust-orange.svg" alt="Language">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
</p>

# Rust Terminal Based Jump Platformer Game

A terminal-based jump platformer game built with Rust.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Docker](#docker)
- [Files](#files)
- [License](#license)

## Installation

To run the game, you'll need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

## Usage

To run the game, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/username/repo.git
cd repo
```

Then, run the following command:

```bash
cargo run --release
```

## Docker

To run the game using Docker, you'll need to have Docker installed. You can install Docker by following the instructions on the [official website](https://docs.docker.com/get-docker/).

To build the Docker image, run the following command:

```bash
docker build -t jump-platformer .
```

To run the game in a Docker container, run the following command:

```bash
docker run -it jump-platformer
```

## Files

- `src/game.rs`: Contains the main game logic.
- `src/player.rs`: Contains the player logic.
- `src/platform.rs`: Contains the platform logic.
- `src/main.rs`: Contains the main function that runs the game.
- `Dockerfile`: Contains the instructions for building the Docker image.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
```

I hope this helps!