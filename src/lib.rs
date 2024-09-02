/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/bit-array-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use std::ops::Index;
use std::vec::Vec;

type BitArrayAtom = u32;
const BIT_ARRAY_BITS_IN_ATOM: usize = 32;

#[derive(Clone)]
pub struct BitArray {
    array: Vec<BitArrayAtom>,
    bit_count: usize,
    number_of_bits_set: usize,
}

impl BitArray {
    /// Initializes a new `BitArray`.
    ///
    /// # Arguments
    ///
    /// * `bit_count` - The maximum number of bits in the array.
    /// # Panics
    ///
    /// This function will panic if `bit_count` is zero.
    #[must_use]
    pub fn new(bit_count: usize) -> Self {
        assert_ne!(bit_count, 0, "bit_count must be greater than zero");
        let atom_count = bit_count.div_ceil(BIT_ARRAY_BITS_IN_ATOM);
        let array = vec![0; atom_count];

        Self {
            array,
            bit_count,
            number_of_bits_set: 0,
        }
    }

    /// Resets all bits in the array.
    pub fn reset(&mut self) {
        self.array.fill(0);
        self.number_of_bits_set = 0;
    }

    /// Checks if all bits are set.
    ///
    /// # Returns
    ///
    /// * `true` if all bits in the array are set, otherwise `false`.
    #[inline]
    #[must_use]
    pub const fn all_set(&self) -> bool {
        self.bit_count == self.number_of_bits_set
    }

    /// Finds the first bit that is not set in the array.
    ///
    /// # Returns
    ///
    /// * The index of the first unset bit, or `None` if all bits are set.
    #[must_use]
    pub fn first_unset_bit(&self) -> Option<usize> {
        for (i, &atom) in self.array.iter().enumerate() {
            if atom != u32::MAX {
                return (0..BIT_ARRAY_BITS_IN_ATOM).find_map(|bit| {
                    if atom & (1 << bit) == 0 {
                        Some(i * BIT_ARRAY_BITS_IN_ATOM + bit)
                    } else {
                        None
                    }
                });
            }
        }
        None
    }

    /// Finds the first bit that is set in the array.
    ///
    /// # Returns
    ///
    /// * The index of the first set bit, or `None` if no bits are set.
    #[must_use]
    pub fn first_set_bit(&self) -> Option<usize> {
        for (i, &atom) in self.array.iter().enumerate() {
            if atom != 0 {
                return (0..BIT_ARRAY_BITS_IN_ATOM).find_map(|bit| {
                    if atom & (1 << bit) != 0 {
                        Some(i * BIT_ARRAY_BITS_IN_ATOM + bit)
                    } else {
                        None
                    }
                });
            }
        }
        None
    }

    /// Returns the number of bits that are currently set to `1`.
    ///
    /// # Returns
    ///
    /// The number of bits that are set in the `BitArray`.
    #[inline]
    #[must_use]
    pub const fn count_set_bits(&self) -> usize {
        self.number_of_bits_set
    }

    /// Returns the total number of bits in the `BitArray`.
    ///
    /// # Returns
    ///
    /// The total number of bits in the `BitArray`.
    #[inline]
    #[must_use]
    pub const fn bit_count(&self) -> usize {
        self.bit_count
    }

    /// Sets the bit at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the bit to set.
    ///
    /// # Panics
    ///
    /// This function will panic if the index is out of bounds.
    #[inline]
    pub fn set(&mut self, index: usize) {
        assert!(index < self.bit_count, "Index out of bounds");

        let array_index = index / BIT_ARRAY_BITS_IN_ATOM;
        let bit_index = index % BIT_ARRAY_BITS_IN_ATOM;
        let mask = 1 << bit_index;

        if self.array[array_index] & mask == 0 {
            self.number_of_bits_set += 1;
        }

        self.array[array_index] |= mask;
    }

    /// Unsets (clears) the bit at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the bit to clear.
    ///
    /// # Panics
    ///
    /// This function will panic if the index is out of bounds.
    #[inline]
    pub fn unset(&mut self, index: usize) {
        assert!(index < self.bit_count, "Index out of bounds");

        let array_index = index / BIT_ARRAY_BITS_IN_ATOM;
        let bit_index = index % BIT_ARRAY_BITS_IN_ATOM;
        let mask = 1 << bit_index;

        if self.array[array_index] & mask != 0 {
            self.number_of_bits_set -= 1;
        }

        self.array[array_index] &= !mask;
    }

    /// Sets or unsets the bit at the given index based on the value of `set`.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the bit to modify.
    /// * `set` - If `true`, the bit will be set (1). If `false`, the bit will be unset (0).
    ///
    /// # Panics
    ///
    /// This function will panic if the index is out of bounds.
    pub fn set_bit(&mut self, index: usize, set: bool) {
        assert!(index < self.bit_count, "Index out of bounds");

        let array_index = index / BIT_ARRAY_BITS_IN_ATOM;
        let bit_index = index % BIT_ARRAY_BITS_IN_ATOM;
        let mask = 1 << bit_index;

        if set {
            if self.array[array_index] & mask == 0 {
                self.number_of_bits_set += 1;
            }
            self.array[array_index] |= mask;
        } else {
            if self.array[array_index] & mask != 0 {
                self.number_of_bits_set -= 1;
            }
            self.array[array_index] &= !mask;
        }
    }

    /// Returns the atom value that is located at the specified index.
    ///
    /// # Arguments
    ///
    /// * `from_index` - The index from which to start reading.
    ///
    /// # Returns
    ///
    /// The atom value at the specified index.
    #[must_use]
    pub fn atom_from_index(&self, from_index: usize) -> BitArrayAtom {
        let mut result = 0;

        for i in 0..BIT_ARRAY_BITS_IN_ATOM {
            let index = from_index + (BIT_ARRAY_BITS_IN_ATOM - 1) - i;
            result <<= 1;
            if index < self.bit_count {
                result |= u32::from(self.get(index));
            }
        }

        result
    }

    /// Returns the bit value at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The bit index to read from.
    ///
    /// # Returns
    ///
    /// The read bit value (0 or 1).
    ///
    /// # Panics
    ///
    /// This function will panic if the index is out of bounds.
    #[must_use]
    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.bit_count, "Index out of bounds");

        let array_index = index / BIT_ARRAY_BITS_IN_ATOM;
        let bit_index = index % BIT_ARRAY_BITS_IN_ATOM;

        ((self.array[array_index] >> bit_index) & 0x1) != 0
    }
}

impl Index<usize> for BitArray {
    type Output = bool;
    /// Provides indexed access to individual bits in the `BitArray`.
    ///
    /// # Arguments
    ///
    /// * `index` - The zero-based index of the bit to access.
    ///
    /// # Returns
    ///
    /// A reference to a boolean value (`true` or `false`) representing the state of the bit
    /// at the specified index.
    ///
    /// # Panics
    ///
    /// This function will panic if the `index` is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use bit_array_rs::BitArray;
    /// let mut bit_array = BitArray::new(16);
    /// bit_array.set(3);
    /// assert_eq!(bit_array[3], true);
    /// assert_eq!(bit_array[0], false);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        if self.get(index) {
            &true
        } else {
            &false
        }
    }
}

impl std::fmt::Debug for BitArray {
    /// Formats the `BitArray` as a binary string with groups of 8 bits separated by a space.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter used to output the debug string.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful.
    ///
    /// # Example
    ///
    /// ```
    /// use bit_array_rs::BitArray;
    /// let mut bit_array = BitArray::new(16);
    /// bit_array.set(3);
    /// bit_array.set(7);
    /// bit_array.set(9);
    /// bit_array.set(15);
    ///
    /// assert_eq!(format!("{:?}", bit_array), "00010001 01000001");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.bit_count {
            if i > 0 && i % 8 == 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", u8::from(self.get(i)))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for BitArray {
    /// Formats the `BitArray` as a continuous binary string without any spaces.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter used to output the display string.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the formatting was successful.
    ///
    /// # Example
    ///
    /// ```
    /// use bit_array_rs::BitArray;
    /// let mut bit_array = BitArray::new(16);
    /// bit_array.set(3);
    /// bit_array.set(7);
    /// bit_array.set(9);
    /// bit_array.set(15);
    ///
    /// assert_eq!(format!("{}", bit_array), "0001000101000001");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.bit_count {
            write!(f, "{}", u8::from(self.get(i)))?;
        }
        Ok(())
    }
}
