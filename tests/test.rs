/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/bit-array-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use bit_array_rs::BitArray;

#[test]
fn basic_bit_array_functions() {
    let mut array = BitArray::new(10);

    array.set(4);

    assert_eq!(array.first_unset_bit().unwrap(), 0);
    assert_eq!(array.first_set_bit().unwrap(), 4);
    assert_eq!(array.bit_count(), 10);
    assert_eq!(array[1], false);
    assert_eq!(array[0], false);
    assert_eq!(array[4], true);
    array.unset(4);
    assert_eq!(array.first_set_bit(), None);
    assert_eq!(array[4], false);
    array.set_bit(9, true);
    assert_eq!(array[9], true);
    assert_eq!(array.get(8), false);
    assert_eq!(array.get(9), true);
    assert_eq!(array.all_set(), false);

    println!("{}", array);
    println!("{:?}", array);
}

#[test]
fn bitarray_debug_output() {
    let mut bit_array = BitArray::new(16);
    bit_array.set(3);
    bit_array.set(7);
    bit_array.set(9);
    bit_array.set(15);

    let output = format!("{:?}", bit_array);
    const EXPECTED_OUTPUT: &str = "00010001 01000001";

    assert_eq!(output, EXPECTED_OUTPUT);
}
