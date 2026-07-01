mod gf256;
use crate::gf256::get_tables;
fn main() {
    let (exp, log) = get_tables();

    for i in 0..256 {
        if exp[i] ==10 {
            println!("{}", i);
        }
    }
    
}
