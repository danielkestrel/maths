# Contributing to Planet Distance Calculator

Thanks for your interest in contributing! This project welcomes contributions of all kinds.

## Ways to Contribute

- 🐛 **Bug Reports** — Found something wrong? Open an issue
- ✨ **Feature Requests** — Have an idea? Let's discuss it
- 📝 **Documentation** — Improvements to README, comments, or guides
- 🔧 **Code** — Bug fixes, new features, or refactoring

## Getting Started

1. **Fork** the repository
2. **Clone** your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/maths.git
   cd maths/kepler_3rd_law
   ```
3. **Build** the project:
   ```bash
   cargo build
   ```
4. **Run tests** (if any):
   ```bash
   cargo test
   ```

## Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes

3. Ensure code compiles without warnings:
   ```bash
   cargo build
   cargo clippy
   ```

4. Format your code:
   ```bash
   cargo fmt
   ```

5. Commit with a clear message:
   ```bash
   git commit -m "Add support for custom star masses"
   ```

6. Push and open a Pull Request

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Add comments for complex physics calculations

## Ideas for Contributions

- [ ] Support for custom star masses (not just our Sun)
- [ ] Reverse calculation: distance → orbital period
- [ ] Support for elliptical orbits (semi-major axis)
- [ ] Add unit tests for edge cases
- [ ] Calculate orbital velocity
- [ ] JSON output format

## Questions?

Open an issue — happy to help!
