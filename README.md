# Offset calculator thing in Rust

## Requirements

- rustc
- cargo

Install:

```
brew install rust
```

## Compiling

```
cargo build --release
```

## Usage

Compile, or use the precompiled one in `./bin`. For ARM Mac!

Precompiled:
```
./bin/calculator 10 20
```

Compile and run:

```
cargo run --quiet --release 10 20
```

## Testing

Compare with the results of `plotpixel.rb`:

```
./test.sh
```
