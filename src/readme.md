# Streamlined NTRU Prime
---

A rust implementation of Streamlined NTRU Prime 4591<sup>761</sup>

NTRU Prime is a lattice based cryptosystem aiming to improve the security of lattice schemes at minimal cost. It is thought to be resistant to quantum computing advances, in particular Shor's algorithm and is an entrant in NIST's Post Quantum Cryptography competition[<sup>[1]</sup>](https://csrc.nist.gov/Projects/Post-Quantum-Cryptography). 

Please read the [warnings](#warnings) before use.

 The algorithm was authored by Daniel J. Bernstein, Chitchanok Chuengsatiansup, Tanja Lange & Christine van Vredendaal. 

 Contributions welcome. SIMD or WASM development especially.
 

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
---

Add as a dependency to cargo.toml
```rust
[dependencies]
streamlined_ntru_prime = "0.1.0"
```

## Usage
---

```rust
use streamlined_ntru_prime::*;

// Key Generation
let (public_key, private_key) = generate_key().expect("Key generation error");

// Encapsulation
let (cipher_text, shared_secret) = encapsulate(public_key).expect("Encapsulation error");

// Decapsulation
let shared_secret = decapsulate(cipher_text, private_key).expect("Decapsulation error")
```

## Testing 
---
To reduce compile time/size tests and benches are an optional feature.
```shell
# Testing
cargo test --features testing

# Benchmarking
cargo bench --features testing
```

Full output of generated and expected values can be shown optionally.
```shell
cargo test -- --nocapture --features testing
```

A json file of 100 KATs was generated from the sage implementation found [here](src/tests/kat-generator.sage) (additional code added to generate json, original [here]()) and used to test encapsulation/decapsulation. Key generation is tested separately.


## Warnings
---

#### Implementation 
This implementation has not undergone any security auditing and while care has been taken no guarantees can be made for either correctness or the constant time running of the underlying functions at compile time. **Please use at your own risk.**

#### Algorithm

Streamlined NTRU Prime was first published in 2016, the C implementation upon which this is based was published in August 2017. The algorithm still requires careful security review. Please see [here](https://ntruprime.cr.yp.to/warnings.html) for further warnings from the authors regarding NTRU Prime and lattice based encryption schemes.