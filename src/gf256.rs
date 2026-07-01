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
            let log_c = (((log[a as usize] as u16)+ (log[b as usize] as u16)) as u8)%255;
            exp[log_c as usize]
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

pub fn get_tables() -> &'static([u8; 256], [u8; 256]) {
    TABLES.get_or_init(|| generate_tables())
}
fn generate_tables() -> ([u8; 256], [u8; 256]) {
    let mut exp = [0u8; 256];
    let mut log = [0u8; 256];

    exp[0] = 1;

    /*
     * ethereum uses 0x11b (x8+x4+x3+x+1) as the irreducible polynomial in GF(256)
     * that gives a cycle length of 51 here. AES systems usually use this polynomial,
     * but use a generator that is primitive for it. The klaudpost/ReedSolomon Go 
     * libray uses 0x11d (x8+x4+x3+x2+1) which gives a cycle length of 255.
     */
    for i in 1..exp.len()-1 {
        let val = (exp[i-1] as u16)<<1; // since 0x11b doesnt fit in u8.. 100011011 is 9 bit btw
        if val > 0xff {
            exp[i] = (val^(0x11d)) as u8;
        } else {
            exp[i] = val as u8;
        }
    }

    // log[0] is not defined, and exp[i] is never 0.
    for i in 0..log.len()-1 {
        log[exp[i] as usize] = i as u8;
    }

    (exp, log)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(GF256::add(213, 1), 212);
        assert_eq!(GF256::add(255, 7), 248);
        assert_eq!(GF256::add(0x2c, 0x32), 30);
    }

    #[test]
    fn test_mul() {
        assert_eq!(GF256::mul(0, 0), 0);
        assert_eq!(GF256::mul(10, 1), 10);
        assert_eq!(GF256::mul(204, GF256::inv(204).unwrap()), 1);
        assert_eq!(GF256::mul(55, 12), GF256::mul(12, 55));
    }
}