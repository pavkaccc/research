# Benchmark: Python (pydivkit) vs Rust (divkit-json-builder)

## Environment

- Platform: Linux 4.4.0
- Python: 3.11, pydivkit 32.35.0
- Rust: release build (optimized)

## Results

| Benchmark | Python (us) | Rust (us) | Speedup |
|-----------|------------|-----------|---------|
| Simple DivText | 474.2 | 0.3 | **1,581x** |
| Nested Slider | 660.1 | 11.9 | **55x** |
| Gallery 100 items | 45,508.5 | 78.3 | **581x** |
| Gallery 1000 items | 454,766.6 | 839.2 | **542x** |
| Nested containers (depth=20) | 9,846.0 | 130.7 | **75x** |
| Nested containers (depth=50) | 27,846.2 | 852.9 | **33x** |
| Complex layout (20 cards) | 28,143.4 | 164.9 | **171x** |
| Schema generation | 39,648.3 | 9.0 | **4,405x** |
| JSON serialization | 120.5 | 13.5 | **9x** |

## Key Takeaways

- **33x to 4,405x faster** across all benchmarks
- Simple operations (DivText, schema) see the biggest gains: **1,000x+**
- Complex nested structures: **33-581x** faster
- JSON serialization alone (serde_json vs json.dumps on already-built dict): **9x** faster
- The bulk of Python's overhead is in object construction and `dict()` traversal, not JSON serialization
