# Digital Filter
                                                                                                
This crate will provide an interface to a digital FIR filter implementation for no-std
environments that cannot depend on a heap being present. By passing in the filter weights and a
matching empty buffer, this crate will instantiate a digital filter that will accept a stream
of inputs and produce a stream of filtered outputs.
                                                                                                
This crate is effectively an implementation of the `lfilter` function in SciPy. The goal of
this crate is to be a self-contained way to apply a digital filter in an embedded system. It
doesn't perform filter design, so you'll need an external tool to design the filter weights for
input. SciPy and Matlab both have excellent tools for this (`scipy.signal.firwin` for SciPy).
