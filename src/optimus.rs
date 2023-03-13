use crate::error::OptimusError;

pub const MAX_INT: u64 = i32::MAX as u64;

///Optimus is used to encode and decode integers using Knuth's Hashing Algorithm.
#[derive(Debug, Clone, Copy)]
pub struct Optimus {
    prime: u64,
    mod_inverse: u64,
    random: u64,
}

impl Optimus {
    /// Returns an Optimus struct that can be used to encode and decode integers.
    /// A common use case is for obfuscating internal ids of database primary keys.
    /// It is imperative that you keep a record of prime, modInverse and random so that
    /// you can decode an encoded integer correctly. random must be an integer less than `MAX_INT`.
    ///
    /// # Errors
    ///
    /// Will return `OptimusError` if the argument `prime` is not prime
    ///
    /// CAUTION: DO NOT DIVULGE prime, modInverse and random!
    pub fn new(prime: u64, mod_inverse: u64, random: u64) -> Result<Self, OptimusError> {
        if !primal_check::miller_rabin(prime) {
            return Err(OptimusError::NotPrime);
        }
        Ok(Self {
            prime,
            mod_inverse,
            random,
        })
    }
    ///Returns an Optimus struct that can be used to encode and decode integers.
    ///random must be an integer less than `MAX_INT`.
    ///It automatically calculates prime's mod inverse and then calls new.
    /// # Errors
    ///
    /// Will return `OptimusError` if the argument `prime` is not prime
    /// or if a Mod Inverse cannot be found
    ///
    pub fn new_calculated(prime: u64, random: u64) -> Result<Self, OptimusError> {
        Self::new(prime, Self::calc_mod_inverse(prime as i64)?, random)
    }
    ///returns the modular inverse of a given prime number.
    ///The modular inverse is defined such that
    ///(`PRIME` * `MODULAR_INVERSE`) & (`MAX_INT`) = 1.
    ///
    ///See: <http://en.wikipedia.org/wiki/Modular_multiplicative_inverse>
    ///
    ///NOTE: prime is assumed to be a valid prime. If prime is outside the bounds of
    ///an i64, then the function panics as it can not calculate the mod inverse.
    /// # Errors
    /// Will return `OptimusError` if the argument `prime` is not prime
    /// or if a mod inverse cannot be found
    ///
    pub fn calc_mod_inverse(prime: i64) -> Result<u64, OptimusError> {
        const MAX: i64 = (MAX_INT + 1) as i64;
        if !primal_check::miller_rabin(prime as u64) {
            return Err(OptimusError::NotPrime);
        }
        Ok(modinverse::modinverse(prime, MAX).ok_or(OptimusError::NoModInverse)? as u64)
    }
    ///Encodes n using Knuth's hashing algorithm.
    pub fn encode(&self, n: u64) -> u64 {
        ((n * self.prime) & MAX_INT) ^ self.random
    }
    ///Decodes n back to the original. It will only decode correctly if the Optimus struct
    ///is consistent with what was used to encode n.
    pub fn decode(&self, n: u64) -> u64 {
        ((n ^ self.random) * self.mod_inverse) & MAX_INT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_calc_mod_inverse() {
        let prime = 309779747;
        let expected_mod_inverse = 49560203;
        let calculated = Optimus::calc_mod_inverse(prime).unwrap();
        assert_eq!(
            calculated, expected_mod_inverse,
            "mod inverse incorrect. Expected={}, Actual={}",
            expected_mod_inverse, calculated
        );
    }
    /// Tests if the encoding process correctly decodes the id back to the original
    #[test]
    fn test_encode() {
        let mut rng = rand::thread_rng();
        let os = [
            Optimus::new(309779747, 49560203, 57733611).unwrap(),
            Optimus::new(684934207, 1505143743, 846034763).unwrap(),
            Optimus::new(743534599, 1356791223, 1336232185).unwrap(),
            Optimus::new(54661037, 1342843941, 576322863).unwrap(),
            Optimus::new(198194831, 229517423, 459462336).unwrap(),
            Optimus::new_calculated(198194831, 459462336).unwrap(),
        ];
        println!("{:?}", os[1]);
        for o in os {
            let c = 10;
            let h = 100; // How many random numbers to select in between 0-c and (MAX_INT-c) - MAX-INT
            let mut vars = vec![];

            for t in 0..c {
                vars.push(t);
            }
            for _ in 0..h {
                let upper = MAX_INT - 2 * c;
                let rand = rng.gen_range(0..upper);
                vars.push(rand + c);
            }

            for t in (MAX_INT - c..MAX_INT).rev() {
                vars.push(t);
            }
            for value in vars {
                let orig = value;
                let hashed = o.encode(value);
                let unhashed = o.decode(hashed);
                println!("%{orig}: %{hashed} -> %{unhashed}");
                assert_eq!(
                    orig, unhashed,
                    "%{}: %{} -> %{} - FAILED",
                    orig, hashed, unhashed
                )
            }
        }
    }
}
