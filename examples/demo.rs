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
