use std::ops::BitXor;

fn generate_tables() -> ([u8; 256], [u8; 256]) {
    let mut exp = [0u8; 256];
    let mut log = [0u8; 256];

    exp[0] = 1;

    for i in 1..exp.len() {
        let val = (exp[i-1] as u16)<<1;
        if val > 0xff {
            exp[i] = val.bitxor(0x11b) as u8;
        } else {
            exp[i] = val as u8;
        }
    }

    

    todo!()
}