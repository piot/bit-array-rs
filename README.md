# BitArray - A Simple Bit Array Implementation in Rust

`bit-array-rs` is a Rust library that provides a simple and efficient implementation of a bit array. A bit array (or bit vector) is a data structure that compactly stores bits (0s and 1s).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
bit-array-rs = "0.0.3"
```

## Usage

### Creating a BitArray

```rust
use bit_array_rs::BitArray;

fn main() {
    let mut bit_array = BitArray::new(16);
    bit_array.set(3);
    bit_array.set(7);
    println!("{:?}", bit_array);  // Outputs: "00001000 10000000"
    println!("{}", bit_array);    // Outputs: "0000100010000000"
}
```

### Setting and Unsetting Bits

```rust
use bit_array_rs::BitArray;

fn main() {
    let mut bit_array = BitArray::new(8);

    bit_array.set(2);
    bit_array.unset(2);

    assert_eq!(bit_array[2], false);
    bit_array.set_bit(2, true);
    assert_eq!(bit_array[2], true);
}
```

### Finding the First Set/Unset Bit

```rust
use bit_array_rs::BitArray;

fn main() {
    let mut bit_array = BitArray::new(16);

    bit_array.set(3);
    bit_array.set(5);

    assert_eq!(bit_array.first_unset_bit(), Some(0));
    assert_eq!(bit_array.first_set_bit(), Some(3));
}
```

### Debug and Display

```rust
use bit_array_rs::BitArray;

fn main() {
    let mut bit_array = BitArray::new(16);
    bit_array.set(3);
    bit_array.set(7);
    bit_array.set(9);
    bit_array.set(15);

    println!("{:?}", bit_array);  // Outputs: "00010001 01000001"
    println!("{}", bit_array);    // Outputs: "0001000101000001"
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
