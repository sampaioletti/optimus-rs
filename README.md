
# Optimus (Rust)

[![Crates.io](https://img.shields.io/crates/v/optimus)](https://crates.io/crates/optimus)
[![Docs.rs](https://docs.rs/optimus/badge.svg)](https://docs.rs/optimus)

With this library, you can transform your internal id's to obfuscated integers based on Knuth's integer hash. It is similar to Hashids, but will generate integers instead of random strings.

## Usage

To get started you will need 3 things;

- Large prime number lower than `2147483647`
- The inverse prime so that `(PRIME * INVERSE) & MAXID == 1`
- A large random integer lower than `2147483647`

```rust
use optimus::Optimus;
pub fn main() {
    let prime = 1580030173; //choose this, but make sure its prime
    let mod_inverse = 59260789; //calculate from prime or use Optimus::new_calculated
    let random = 1163945558; //choose this at random
    let opt = Optimus::new(prime, mod_inverse, random).unwrap();
    //or
    // let opt = Optimus::new_calculated(prime, random).unwrap();
    // to calculate mod_inverse
    let id = 15; //ID we want to obfuscate
    let encoded_id = opt.encode(id); //obfuscated id=1103647397
    assert_eq!(encoded_id, 1103647397);
    let decoded_id = opt.decode(encoded_id); //back to 15
    assert_eq!(id, decoded_id, "Id and decoded ID should be the same")
}
```

## Inspiration

This package is based on the Go library by [pjebs](https://github.com/pjebs/optimus-go).

Which is in turn based on the PHP library by [jenssegers](https://github.com/jenssegers/optimus).
