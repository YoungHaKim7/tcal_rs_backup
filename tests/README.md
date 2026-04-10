# test(echo)

```bash
echo "0x1f3dc" | ./target/debug/tcal_rs)
  ⎿             127,964
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
     HEX : "0x1F3DC"
     … +8 lines (ctrl+o to expand)


⏺ Bash(echo "0o755" | ./target/debug/tcal_rs)  
```
# test
The Makefile is now working. Here's a summary:

**Created `/home/gy/utilities/qcal_rs/tests/Makefile`** - A test runner that:
1. Builds the Rust project with `cargo build --release`
2. Runs each `.batch` test file by piping expressions to the `qcal_rs` binary
3. Compares actual output against expected output
4. Reports pass/fail results

**Modified `/home/gy/utilities/qcal_rs/src/main.rs`** - Added `atty` check to suppress header output when running non-interactively (for batch testing)

**Added `atty` dependency** to `Cargo.toml`

**Usage:**
```bash
cd /home/gy/utilities/qcal_rs/tests
make               # Run all tests
make parser.batch  # Run specific test file
make clean         # Clean build artifacts
make check         # Run cargo tests
```

**Test Results:**
- `operators.batch`: 10/30 passed
- `parser.batch`: 15/27 passed
- `bitwise.batch`: 11/24 passed

The failures are expected since `qcal_rs` is a new Rust implementation that doesn't yet support all the features of the original libqalculate (natural language operators, Unicode symbols, unit suffixes, factorial, etc.).
