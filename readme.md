# Streamlined NTRU Prime

[![Crates.io](https://img.shields.io/crates/v/streamlined-ntru-prime.svg)](https://crates.io/crates/streamlined-ntru-prime) [![Build Status](https://travis-ci.com/MitchellBerry/Streamlined-NTRU-Prime.svg?branch=master)](https://travis-ci.com/MitchellBerry/Streamlined-NTRU-Prime) ![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg)
[![dependency status](https://deps.rs/crate/streamlined-ntru-prime/0.1.2/status.svg)](https://deps.rs/crate/streamlined-ntru-prime/0.1.2)

## Update warning 

This code is based on the first round submission and outdated. The C codebase has been widely refactored and multiple parameter sets are now available. This library is similar to the current second round sntrup761 submission.

Don't use this library in production.

Bringing it into line with the current NTRUPrime is an aspirational todo. Probably will wait until the NIST Post-Quantum competition is finished. 

----

A rust implementation of Streamlined NTRU Prime 4591<sup>761</sup>

NTRU Prime is a lattice based cryptosystem aiming to improve the security of lattice schemes at minimal cost. It is thought to be resistant to quantum computing advances, in particular Shor's algorithm and is an entrant in NIST's Post Quantum Cryptography competition[<sup>[1]</sup>](https://csrc.nist.gov/Projects/Post-Quantum-Cryptography). 

Please read the [warnings](#warnings) before use.

 The algorithm was authored by Daniel J. Bernstein, Chitchanok Chuengsatiansup, Tanja Lange & Christine van Vredendaal. 

 Contributions welcome. SIMD especially. 
 
 WASM is functional on a seperate branch and not in master.
 

#### Parameter set:
* *p = 761* 
* *q = 4591*
* *w = 286*


#### Outputs:

|    Type   	| Bytes 	|
|---------------|----------:|
|  Public Key 	|  1218 	|
| Private Key 	|  1600 	|
|  Ciphertext 	|  1047 	|
| Shared Key  	|   32  	|

## Installation

Add to your cargo.toml file
```rust
[dependencies]
streamlined_ntru_prime = "0.1.2"
```

## Usage

```rust
use streamlined_ntru_prime::*;

// Key Generation
let (public_key, private_key) = generate_key();

// Encapsulation
let (cipher_text, encapsulated_shared_secret) = encapsulate(public_key);

// Decapsulation
let decapsulated_shared_secret = decapsulate(cipher_text, private_key).expect("Decapsulation failure");

assert_eq!(encapsulated_shared_secret, decapsulated_shared_secret);
```

#### Current Benchmarks

Tested on an  Intel i7-7500U @ 2.7GHz
```shell
running 3 tests
test decapsulate_bench ... bench:   8,785,535 ns/iter (+/- 27,291)
test encapsulate_bench ... bench:   3,215,100 ns/iter (+/- 30,317)
test key_gen_bench     ... bench:  16,914,970 ns/iter (+/- 278,949)
```

## Warnings

#### Implementation 
This implementation has not undergone any security auditing and while care has been taken no guarantees can be made for either correctness or the constant time running of the underlying functions. **Please use at your own risk.**

#### Algorithm

Streamlined NTRU Prime was first published in 2016, the C implementation upon which this is based was published in August 2017. The algorithm still requires careful security review. Please see [here](https://ntruprime.cr.yp.to/warnings.html) for further warnings from the authors regarding NTRU Prime and lattice based encryption schemes.
