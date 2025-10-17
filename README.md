# PHONE DETECTOR

A **production-grade phone number detection library** written in Rust with performance, security, and correctness as top priorities. Designed to find various phone number formats including domestic US numbers, international numbers, toll-free numbers, and mobile numbers within text.

Built with: Rust (stable) + Cargo. This repository uses Cargo for dependency management and building.

---

## ✨ Features

* **Multi-Format Support** – Detects formatted domestic numbers, toll-free numbers, international numbers (with + prefix), plain digit sequences, and mobile numbers
* **High-Performance Scanning** – Utilizes lookup tables, efficient scanning algorithms, and zero-cost abstractions achieving **2M+ operations/second**
* **Thread-Safe by Default** – Rust's ownership model ensures safe concurrent usage (`Send`/`Sync` traits)
* **Security Hardened** – Input size limits (10MB max) to prevent Denial of Service (DoS) attacks, safe parsing with no buffer overflows
* **SOLID Principles** – Code structured using trait-based design for maintainability and extensibility
* **Duplicate & Overlap-Free** – Extracts unique, non-overlapping phone numbers from text
* **Space-Separated Number Detection** – Intelligently handles space-separated formats (e.g., `99 88 77 66 55`)
* **Comprehensive Test Suite** – Includes validation tests, scanning tests, and a multi-threaded performance benchmark
* **Panic Safe** – Graceful error handling with Rust's type system, no undefined behavior

---

## 📌 Supported Phone Formats

| Format Type | Example | Description |
|------------|---------|-------------|
| `FORMATTED_DOMESTIC` | `(123) 456-7890`<br>`123-456-7890`<br>`123.456.7890` | US/Canadian formatted numbers with parentheses, dashes, or dots |
| `FORMATTED_TOLL_FREE` | `1-800-555-1234`<br>`1.800.555.1234` | 11-digit toll-free numbers starting with 1 |
| `INTERNATIONAL_PLUS` | `+1 123-456-7890`<br>`+91-9876543210`<br>`+44 20 1234 5678`<br>`+1 (415) 555-0182` | International format with + prefix |
| `PLAIN_10_DIGIT` | `2345678901` | 10-digit plain numbers without separators |
| `PLAIN_11_DIGIT` | `12345678901` | 11-digit plain numbers starting with 1 |
| `MOBILE_10_DIGIT` | `9876543210`<br>`99887 76655`<br>`998 877 6655` | Mobile numbers starting with 1-9, with or without space separators |

---

## 📌 Use Cases

* Scan documents, emails, or logs for phone numbers
* Extract contact information from unstructured text
* Data validation and normalization pipelines
* Customer data extraction and processing systems
* Compliance and data discovery (e.g., PII detection)
* Lead generation and contact information mining
* Privacy scanning and redaction systems

---

## 🚀 Included Components (Project Layout)

This project is a single-binary Cargo crate:

```
phone_detector/
├─ Cargo.toml
├─ Cargo.lock
├─ .gitignore
├─ README.md
└─ src/
   └─ main.rs      # entry point with all detection logic
```

**Core Components:**
* `PhoneScanner` – The core detection and extraction logic with optimized scanning algorithms
* `PhoneValidator` trait – Interface for individual phone validators
* Individual validators for each phone type (domestic, international, mobile, toll-free)
* `CharacterClassifier` – Ultra-fast character classification using lookup tables
* `PhoneDetectorFactory` – Factory pattern for creating scanner and validator instances
* `PhoneMatch` – Structured result containing type, value, normalized digits, and position
* Validation, scanning, and performance test suites in `main()`

---

## 🔧 Quick Start (Cargo)

From the project root, use these commands to build and run:

### Development build

```bash
cargo build
```

### Development run

```bash
cargo run
```

### Release (optimized) build

```bash
cargo build --release
```

### Release run

```bash
cargo run --release
```

> `cargo build --release` enables optimizations (equivalent to `-O3`) and yields an executable in `target/release/`.

**Performance:** Achieves **~2.87M operations/second** on modern hardware with release builds (16 threads).

---

## 🔌 Dependencies

Add required dependencies to `Cargo.toml`:

```toml
[dependencies]
# No external dependencies required for core functionality
# Optional for benchmarking:
# num_cpus = "1.17.0"  # Auto-detect CPU cores for benchmarks
```

The core library has **zero external dependencies**. For the benchmark suite, you may optionally add `num_cpus` to automatically detect available CPU cores.

After editing `Cargo.toml`, run `cargo build` to fetch and compile dependencies.

---

## ▶️ Running the Program

### Linux / macOS

```bash
cargo run --release
# or the built executable
./target/release/phone_detector
```

### Windows (PowerShell)

```powershell
cargo run --release
# or the built exe
.\target\release\phone_detector.exe
```

### Windows (CMD)

```cmd
cargo run --release
target\release\phone_detector.exe
```

Replace `phone_detector` with the `name` field from your `Cargo.toml` (defaults to the package directory name).

---

## 📊 Expected Output

Running the program executes validation tests, scanning tests, a live demo, and a high-throughput performance benchmark:

```
====================================================================================================
=== PHONE VALIDATION TESTS ===
====================================================================================================

✓ Formatted with parentheses
✓ Formatted with dashes
✓ Formatted with dots
✓ Invalid area code (starts with 0)
✓ Plain 10 digits
✓ Plain 11 digits with 1
✓ Invalid area code
✓ International format
✓ International mobile format
✓ International format
✓ Mobile 10 digits
✓ Mobile with country code
✓ Valid mobile (starts with 5)

Result: 13/13 passed (100%)


====================================================================================================
=== PHONE SCANNING TESTS ===
====================================================================================================

✓ Formatted in text
  Found 1 phone(s)
    [FORMATTED_DOMESTIC] (123) 456-7890 (normalized: 1234567890)

✓ Multiple formatted numbers
  Found 2 phone(s)
    [FORMATTED_DOMESTIC] 123-456-7890 (normalized: 1234567890)
    [FORMATTED_DOMESTIC] 987-654-3210 (normalized: 9876543210)

✓ International format
  Found 1 phone(s)
    [INTERNATIONAL_PLUS] +91 9876543210 (normalized: 919876543210)

✓ Mixed formats
  Found 2 phone(s)
    [INTERNATIONAL_PLUS] +1 234-567-8900 (normalized: 12345678900)
    [MOBILE_10_DIGIT] 9876543210 (normalized: 9876543210)

✓ Space-separated mobile
  Found 1 phone(s)
    [MOBILE_10_DIGIT] 99887 76655 (normalized: 9988776655)

✓ Story: International and spaced mobile
  Found 2 phone(s)
    [INTERNATIONAL_PLUS] +44 20 7946 0123 (normalized: 442079460123)
    [MOBILE_10_DIGIT] 98765 43210 (normalized: 9876543210)

✓ Story: International and toll-free
  Found 2 phone(s)
    [INTERNATIONAL_PLUS] +1 (415) 555-0182 (normalized: 14155550182)
    [FORMATTED_TOLL_FREE] 1-800-555-0199 (normalized: 18005550199)

Result: 20/20 passed (100%)


====================================================================================================
=== PHONE DETECTION DEMO ===
====================================================================================================

Found 6 phone numbers:

  [FORMATTED_DOMESTIC] at pos 14
  Value: (234) 567-8900
  Normalized: 2345678900

  [INTERNATIONAL_PLUS] at pos 32
  Value: +91-9876543210
  Normalized: 919876543210

  [FORMATTED_DOMESTIC] at pos 56
  Value: 345-678-9012
  Normalized: 3456789012

  [MOBILE_10_DIGIT] at pos 78
  Value: 9123456789
  Normalized: 9123456789

  [MOBILE_10_DIGIT] at pos 95
  Value: 99887 76655
  Normalized: 9988776655

  [INTERNATIONAL_PLUS] at pos 114
  Value: +1 (234) 567-8900
  Normalized: 12345678900


====================================================================================================
=== PERFORMANCE BENCHMARK ===
====================================================================================================
Threads: 16
Iterations per thread: 100000
Test cases: 14
Total operations: 22400000

Starting benchmark...

----------------------------------------------------------------------------------------------------
RESULTS:
----------------------------------------------------------------------------------------------------
Time: 7802 ms
Ops/sec: 2871058
Total phones found: 28800000
====================================================================================================


====================================================================================================
✓ SOLID Principles Applied
✓ Optimized for 1M+ ops/sec Performance
✓ Character Classification Lookup Tables
✓ Thread-Safe Implementation
✓ Multiple Phone Format Support
✓ Space-Separated Number Detection
✓ Generic Country-Independent Detection
====================================================================================================
```

---

## 🧪 Testing & Benchmarks

The program includes a self-contained test suite that runs automatically:

* **Validation Tests:** Verifies that each `PhoneValidator` correctly identifies valid and invalid phone number formats based on rules like:
  - Area code validation (no leading 0, first digit 2-9)
  - Exchange code validation (must be ≥2)
  - Digit count requirements
* **Scanning Tests:** Ensures the `PhoneScanner` can accurately find and extract phone numbers from various text blocks, including edge cases like:
  - Mixed format documents
  - Space-separated mobile numbers with various spacing patterns
  - International numbers with parentheses
  - Multiple phone numbers in the same text
  - Real-world text scenarios and stories
* **Performance Benchmark:** A multi-threaded stress test that measures the number of scan operations per second on your hardware

Run tests separately with:

```bash
cargo test
```

For additional benchmarking with Criterion (if added):

```bash
cargo bench
```

---

## 🎯 Key Technical Features

### Character Classification Optimization
Uses a 256-entry lookup table (`CharacterClassifier::CHAR_TABLE`) for O(1) character classification:
- Digits (0-9)
- Separators (space, dash, dot, parentheses)
- Plus sign (+)

### Rust Safety Guarantees
- **No buffer overflows** – Bounds checking at compile time
- **No null pointer dereferences** – No null pointers in safe Rust
- **No data races** – Enforced by the type system (`Send`/`Sync` traits)
- **Memory safety** – Automatic memory management without garbage collection overhead

### Intelligent Format Detection
The scanner uses a multi-pass approach:
1. **International numbers** (scanned first to handle `+1 (xxx)` patterns correctly)
2. **Formatted numbers** (parentheses, dashes, dots, spaces)
3. **Plain digit sequences** (fallback for unformatted numbers)

### Overlap Prevention
Extracted phone numbers are sorted by position and filtered to ensure no overlapping matches, returning only the first valid match at each position.

### Mobile Number Intelligence
Distinguishes between:
- **Standard formatted**: `987-654-3210` with dashes/dots → `FORMATTED_DOMESTIC`
- **Space-separated mobile**: `99887 76655` with spaces + starts with 1-9 → `MOBILE_10_DIGIT`

---

## 📋 Requirements

* **Rust toolchain:** stable (install via `rustup`). Tested on Rust 1.70+
* **OS:** Linux, macOS, or Windows
* **Hardware:** Any modern CPU. Release builds leverage available optimizations
* **RAM:** Minimal requirements; handles up to 10MB text inputs by default

---

## ⚙️ Configuration

### Input Size Limits
The scanner has built-in safety limits to prevent DoS attacks:
```rust
max_input_size: 10 * 1024 * 1024,  // 10MB
max_phone_length: 30,               // Max phone string length
min_digits: 7,                      // Minimum valid phone digits
max_digits: 15,                     // Maximum valid phone digits
```

These can be adjusted in the `PhoneScanner::new()` constructor if needed for your use case.

### Validation Rules
The detector implements North American Numbering Plan (NANP) rules:
- **Area code (NXX):** First digit 2-9 (N), last two digits any 0-9 (XX)
- **Exchange code (NXX):** First digit 2-9 (N), last two digits any 0-9 (XX)
- **Subscriber number (XXXX):** Any four digits 0-9

These rules can be customized in the individual validator structs.

---

## ⚠️ Important Notes

### Portability and CPU Optimizations

Rust's release builds automatically apply optimizations without CPU-specific flags. For maximum performance with CPU-specific instructions, use:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

**Note:** Binaries built with `target-cpu=native` may not run on older/different CPUs. For portable binaries, use standard release builds without custom `RUSTFLAGS`.

### Thread Count Detection

The benchmark automatically detects available CPU cores using:
```rust
let num_threads = std::thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(4);
```

No external dependencies required for thread detection in Rust 1.70+.

### Windows Compatibility

* `cargo run --release` works directly on all platforms
* On Windows PowerShell, prefix executables with `./` or `.\` (e.g., `.\target\release\phone_detector.exe`)

### Character Encoding

The detector assumes UTF-8/ASCII input. Phone numbers with Unicode characters or RTL (right-to-left) text may not be detected correctly without preprocessing.

### False Positives

The detector is designed to be permissive and may occasionally detect numbers that aren't actually phone numbers (e.g., serial numbers, IDs). For production use, consider:
- Adding context-aware filtering
- Implementing allowlists/denylists
- Validating extracted numbers against phone number databases
- Using libphonenumber bindings for stricter validation

---

## ✅ Security Features (Rust Implementation)

* **Input size validation** – Upper limit (10MB) on text parsed to prevent DoS attacks
* **Memory safety** – No buffer overflows thanks to Rust's ownership model and bounds checking
* **No manual memory management** – Automatic memory safety without garbage collection overhead
* **Thread-safe by default** – Compiler-enforced thread safety via `Send`/`Sync` traits
* **No data races** – Impossible to create data races in safe Rust
* **Panic-safe operations** – Controlled error handling prevents unexpected crashes
* **Zero-cost abstractions** – High performance without sacrificing safety
* **No undefined behavior** – Rust's type system eliminates entire classes of bugs

---

## 🏗️ Architecture Highlights

* **Trait-based design** – `PhoneValidator` trait for extensibility and polymorphism
* **Factory pattern** – `PhoneDetectorFactory` for creating validator instances
* **Single Responsibility Principle** – Each validator handles one phone type
* **Lookup tables** – `CharacterClassifier` for O(1) character classification
* **Efficient scanning** – Optimized algorithms with minimal allocations
* **Memory pooling** – Pre-allocated vectors with `Vec::with_capacity()` reduce runtime allocations
* **Zero-copy where possible** – Uses byte slices (`&[u8]`) instead of string copies

---

## 🚀 Performance Tips

1. **Always use release builds** for production (`cargo build --release`)
2. **Enable CPU-specific optimizations** with `RUSTFLAGS="-C target-cpu=native"`
3. **Enable LTO** (Link-Time Optimization) in `Cargo.toml`:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   ```
4. **Profile with `cargo flamegraph`** to identify bottlenecks
5. **Use `cargo-bloat`** to analyze binary size
6. **Run benchmarks** to measure real-world performance on your hardware
7. **Consider using `&str` instead of `String`** when possible to avoid allocations

---

## 📄 License

This code is provided as-is for educational and commercial use. Feel free to modify and distribute according to your needs.

---

**Built with ❤️ and Rust 🦀 for maximum performance, safety, and reliability**