# Upwork Backend

This is a Rust-based backend application. Follow the steps below to set up and run the application locally.

## Prerequisites

Ensure you have the following installed on your system:
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Getting Started

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/northlas/upwork-backend.git
    cd upwork-backend
    ```

2. **Build the Application**:
    ```bash
    cargo build
    ```

3. **Run the Application**:
    ```bash
    cargo run
    ```

4. **Testing**:
    To run tests, use:
    ```bash
    cargo test
    ```

## Configuration

Create a `.env` file in the root directory and add the necessary configurations. Example:
```
GEMINI_API_URL=your_gemini_url
GEMINI_API_KEY=your_api_key
```

## License