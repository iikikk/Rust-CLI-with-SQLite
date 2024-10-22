# Rust CLI Tool

SQLite is a command-line interface tool built with Rust, enabling efficient management of a simple movie database. This CLI tool allows for operations such as adding, retrieving, updating, deleting, and listing movie records.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [CI/CD Pipeline](#cicd-pipeline)
- [Screenshot](#sreenshot)

## Features

This CLI tool supports the following functionalities:
- **Create**: Add new movie records.
- **Read**: Retrieve details of a specific movie.
- **Update**: Modify existing movie records.
- **Delete**: Remove movie records from the system.
- **List**: Display all movie records.

## Installation

To get started with Rust-CLI, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://yourrepositorylink.com/Rust-CLI.git
   cd Rust-CLI
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Access the executable**:
   Find the executable in the `target/release/` directory after building the project.

## Usage

Execute commands using the syntax below:

- **Help**:
  ```bash
  sqlite help
  ```
  Displays usage and commands.

- **Creating a Movie**:
  ```bash
  sqlite create <id> <title> <director> <release_date>
  ```
  Example:
  ```bash
  sqlite create 1 "Inception" "Christopher Nolan" "2010/7/16"
  ```

- **Reading a Movie**:
  ```bash
  sqlite read <id>
  ```
  Example:
  ```bash
  sqlite read 1
  ```

- **Updating a Movie**:
  ```bash
  sqlite update <id> [title] [director] [release_date]
  ```
  Example:
  ```bash
  sqlite update 1 "Inception" "Christopher Nolan" "2010/7/18"
  ```

- **Deleting a Movie**:
  ```bash
  sqlite delete <id>
  ```
  Example:
  ```bash
  sqlite delete 1
  ```

- **Listing All Movies**:
  ```bash
  sqlite list
  ```

## CI/CD Pipeline
[![CI](https://github.com/iikikk/Rust-CLI/actions/workflows/CI.yml/badge.svg)](https://github.com/iikikk/Rust-CLI/actions/workflows/CI.yml)

Our CI/CD pipeline is set up using GitHub Actions and includes the following steps:

1. **Code Checkout**: Pulls the latest code from the main branch.
2. **Environment Setup**: Installs the latest stable version of Rust.
3. **Linting**: Runs `cargo clippy` to ensure code quality.
4. **Build**: Compiles the project using `cargo build --release`.
5. **Testing**: Executes `cargo test` to run all unit tests.
6. **Artifact Upload**: Uploads the built executable as an artifact in the CI pipeline.

This setup ensures that each push or pull request is automatically built and tested, maintaining high code quality and deployment readiness.

## Screenshot
![result1.png](result1.png)

![result2.png](result2.png)
