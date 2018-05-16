//! # Digital Filter
//!
//! This crate will provide an interface to a digital FIR filter implementation. By passing in the
//! filter weights and a matching empty buffer, this crate will instantiate a digital filter that
//! will accept a vector or stream of inputs and produce a vector or stream of filtered
//! outputs.
//!
//! This crate will *not* try to impose any higher-level meaning on the output of the filter. This
//! means that issues such as filter delay will not be addressed.
//!
//! The primary objective of this crate is to abstract away the process of moving data through a
//! pre-defined digital filter. It doesn't perform filter design, so you'll need an external tool
//! to design the filter weights for input. Matlab has an excellent tool for this, and there are
//! several open-source alternatives as well. A list of possible tools will be added here for
//! reference later.
#![no_std]

use core::ops::IndexMut;

extern crate heapless;
use heapless::RingBuffer;
use heapless::consts::*;

type FilterBuf = [f32; 3];
type FilterRing = RingBuffer<f32, U3>;

struct DigitalFilter {
    coeffs: FilterBuf,
    buffer: FilterRing,
    num_taps: usize
}

impl DigitalFilter {
    fn new(coeffs: FilterBuf, buffer: FilterRing) -> Self {
        let num_taps = coeffs.len();
        DigitalFilter { coeffs, buffer, num_taps }
    }

    fn filter(mut self, input: f32) -> f32 {
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
        let mut coeffs = [1, 1, 1];
        let mut buffer = [0, 0, 0];
        let filter = DigitalFilter::new(coeffs, buffer);
        let inputs = [4, 8, 15, 16, 23, 42];
        let expected_output = [0, 0, 0, 4, 8, 15];
        let actual_output = inputs.iter().map(|item| filter.filter(item)).collect();
        assert_eq!(expected_output, actual_output);
    }

    // #[test]
    // fn sos_filter_test() {
    //     let sos_filter: [[f32; 6]; 5] = [
    //         [1., 2., 1., 1., -1.4818, 0.8316],
    //         [1., 2., 1., 1., -1.2772, 0.5787],
    //         [1., 2., 1., 1., -1.1430, 0.4128],
    //         [1., 2., 1., 1., -1.0619, 0.3126],
    //         [1., 2., 1., 1., -1.0237, 0.2654]
    //     ];
    //     let gain: [f32; 5] = [0.0875, 0.754, 0.0675, 0.0627, 0.0604];
    //     // let filter = DigitalFilter(sos_filter, gain);

    //     let input: [i32; 7] = [0,-1,0,1,0,-1,0];
    //     // let output = input.into_iter().map(|i| filter.process(i));
    //     let mut output = [0; 7];
    //     for (idx, o) in input.into_iter().map(|i| -1 * i).enumerate() {
    //         output[idx] = o;
    //     }
    //     assert_eq!(output, [0,1,0,-1,0,1,0]);
    // }
}
