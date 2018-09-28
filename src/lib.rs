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
type FilterBuf<N> = GenericArray<FilterItem, N>;
type FilterRing<N> = RingBuffer<FilterItem, N>;

pub struct DigitalFilter<N>
where
    N: Add<U1> + ArrayLength<FilterItem>,
    Sum<N, U1>: ArrayLength<FilterItem>
{
    coeffs: FilterBuf<N>,
    buffer: FilterRing<N>
}

impl<N> DigitalFilter<N>
where
    N: Add<U1> + ArrayLength<FilterItem>,
    Sum<N, U1>: ArrayLength<FilterItem>
{
    pub fn new(coeffs: FilterBuf<N>) -> Self {
        let num_taps = coeffs.len();
        let mut buffer: FilterRing<N> = RingBuffer::new();
        for _idx in 0..num_taps {
            buffer.enqueue(0.).unwrap();
        }
        DigitalFilter { coeffs, buffer }
    }


    pub fn filter(&mut self, input: f32) -> f32 {
        let _ = self.buffer.dequeue();
        self.buffer.enqueue(input).unwrap();
        let mut output: f32 = 0_f32;
        let mut c_idx = self.coeffs.len();
        for el in self.buffer.iter() {
            c_idx -= 1;
            output += el * self.coeffs[c_idx];
        }
        output
    }
}


#[cfg(test)]
mod tests {
    use DigitalFilter;

    #[test]
    fn basic_filter_test() {
        let coeffs = arr![f32; 1., 1., 1.];
        let mut filter = DigitalFilter::new(coeffs);
        let inputs = [4., 8., 15., 16., 23., 42.];
        let expected_output = [4., 12., 27., 39., 54., 81.];
        let mut actual_output = [0.; 6];
        for (idx, input) in inputs.iter().enumerate() {
            actual_output[idx] = filter.filter(*input);
        }
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn varying_weight_filter_test() {
        let coeffs = arr![f32; 1., 2., 3.];
        let mut filter = DigitalFilter::new(coeffs);
        let inputs = [4., 8., 15., 16., 23., 42.];
        let expected_output = [4., 16., 43., 70., 100., 136.];
        let mut actual_output = [0.; 6];
        for (idx, input) in inputs.iter().enumerate() {
            actual_output[idx] = filter.filter(*input);
        }
        assert_eq!(expected_output, actual_output);
    }
}
