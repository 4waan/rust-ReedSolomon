use std::sync::OnceLock;

pub struct GF256;

impl GF256 {
    pub fn add(a: u8, b: u8) -> u8 {
        a^b
    }

    pub fn mul(a: u8, b:u8) -> u8 {
        let (exp, log) = get_tables();
        if a==0 || b==0 {
            0
        } else {
            let log_c = (log[a as usize] + log[b as usize]) as u16 ;
            let val = log_c%255;
            exp[val as usize]
        }
    }

    pub fn inv(a: u8) -> Result<u8, String> {
        let (exp, log) = get_tables();
        if a==0 {
            Err("0 has no inverse".to_string())
        } else {
            Ok(exp[(255-log[a as usize]) as usize])
        }
    }

    pub fn div(a: u8, b: u8) -> Result<u8, String> {
        if a==0 {
            Ok(0)
        } else if b==0{
            Err("cant divide by 0".to_string())
        } else {
            Ok(Self::mul(a, Self::inv(b)?))
        }
    }
}

static TABLES: OnceLock<([u8; 256], [u8; 256])> = OnceLock::new();

fn get_tables() -> &'static([u8; 256], [u8; 256]) {
    TABLES.get_or_init(|| generate_tables())
}
fn generate_tables() -> ([u8; 256], [u8; 256]) {
    let mut exp = [0u8; 256];
    let mut log = [0u8; 256];

    exp[0] = 1;

    for i in 1..exp.len() {
        let val = (exp[i-1] as u16)<<1; // since 0x11b doesnt fit in u8.. 100011011 is 9 bit btw
        if val > 0xff {
            exp[i] = (val^(0x11b)) as u8;
        } else {
            exp[i] = val as u8;
        }
    }

    for i in 0..log.len() {
        log[exp[i] as usize] = i as u8;
    }

    (exp, log)
}