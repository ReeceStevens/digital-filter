//! # Digital Filter
//!
//! This crate will provide an interface to a digital FIR filter implementation for no-std
//! environments that cannot depend on a heap being present. By passing in the filter weights and a
//! matching empty buffer, this crate will instantiate a digital filter that will accept a stream
//! of inputs and produce a stream of filtered outputs.
//!
//! This crate is effectively an implementation of the `lfilter` function in SciPy. The goal of
//! this crate is to be a self-contained way to apply a digital filter in an embedded system. It
//! doesn't perform filter design, so you'll need an external tool to design the filter weights for
//! input. SciPy and Matlab both have excellent tools for this (`scipy.signal.firwin` for SciPy).

#![no_std]

use core::ops::IndexMut;

#[macro_use]
extern crate generic_array;
extern crate heapless;
extern crate typenum;

use core::ops::Add;
use heapless::RingBuffer;
use heapless::consts::*;
use generic_array::{GenericArray, ArrayLength};
use typenum::Sum;

type FilterItem = f32;
type FilterBuf<N: ArrayLength<FilterItem>> = GenericArray<FilterItem,N>;
type FilterRing<N: ArrayLength<FilterItem>> = RingBuffer<FilterItem, N>;

struct DigitalFilter<N>
where
    N: Add<U1> + ArrayLength<FilterItem>,
    Sum<N, U1>: ArrayLength<FilterItem>
{
    coeffs: FilterBuf<N>,
    buffer: FilterRing<N>,
    num_taps: usize
}

impl<N> DigitalFilter<N>
where
    N: Add<U1> + ArrayLength<FilterItem>,
    Sum<N, U1>: ArrayLength<FilterItem>
{
    fn new(coeffs: FilterBuf<N>, mut buffer: FilterRing<N>) -> Self {
        let num_taps = coeffs.len();
        // Initialize the buffer
        for idx in 0..num_taps {
            buffer.enqueue(0.).unwrap();
        }
        DigitalFilter { coeffs, buffer, num_taps }
    }

    fn filter(&mut self, input: f32) -> f32 {
        let _ = self.buffer.dequeue();
        self.buffer.enqueue(input).unwrap();
        let mut output: f32 = 0_f32;
        for (idx, el) in self.buffer.iter().enumerate() {
            output += el * self.coeffs[idx];
        }
        output
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn basic_filter_test() {
        use DigitalFilter;
        use heapless::RingBuffer;
        use heapless::consts::{U3,U6};
        let mut coeffs = arr![f32; 1., 1., 1.];
        let mut buffer: RingBuffer<f32,U3> = RingBuffer::new();
        let mut filter = DigitalFilter::new(coeffs, buffer);
        let inputs = [4., 8., 15., 16., 23., 42.];
        let expected_output = [4., 12., 27., 39., 54., 81.];
        let mut actual_output = [0.; 6];
        for (idx, input) in inputs.iter().enumerate() {
            actual_output[idx] = filter.filter(*input);
        }
        assert_eq!(expected_output, actual_output);
    }
}