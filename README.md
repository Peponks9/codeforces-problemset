# Codeforces Problemset Solutions

A collection of solutions to problems from [Codeforces](https://codeforces.com/problemset) implemented in Rust.

## 🚀 Project Structure

This repository contains two types of solutions:

### 1. Standalone Solutions (`solutions/`)

Individual Rust files that can be compiled and run independently. Each file contains a complete solution for a specific Codeforces problem.

### 2. Integrated Solutions (`src/problems/`)

Solutions integrated into the main project structure, using shared utilities and a common I/O system.

## 🛠️ Setup and Usage

### Prerequisites

- Rust (2024 edition or later)
- Cargo

### Running Standalone Solutions

```bash
# Compile and run a specific solution
rustc solutions/problem_name.rs -o problem_name
./problem_name

# Or run directly with cargo
cargo run --bin problem_name
```

### Running Integrated Solutions

```bash
# Build the project
cargo build

# Run the current problem (configured in src/main.rs)
cargo run
```
