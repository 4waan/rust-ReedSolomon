mod gf256;
use crate::gf256::get_tables;
fn main() {
    let (exp, log) = get_tables();

    for i in 0..255 {
    println!("exp[{}] = {}", i, exp[i]);
}
    
}
