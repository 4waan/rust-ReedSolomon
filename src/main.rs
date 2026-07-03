mod gf256;
mod poly;
mod encoder;

use crate::gf256::get_tables;
use poly::Polynomial;
fn main() {
    let coeffs3: Vec<u8> = vec![];

    let poly3 = Polynomial {
            coeffs: coeffs3,
        };

    println!("{:?}", poly3.eval(4));
}
