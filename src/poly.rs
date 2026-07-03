use crate::gf256;

/*
 * To evaluate p(x) at some point z, you compute:
 * p(z) = c₀ + c₁z + c₂z² + c₃z³ + ...
 * There's a nice algorithm for this called Horner's method:
 * p(z) = c₀ + z(c₁ + z(c₂ + z(c₃ + ...z(cn))))
 * this is the direction we build our coeffs vector in to create a polynomial.
*/

// yeah why tf a struct for polynomial? no fuckn idea might undo this shit.
pub struct Polynomial {
    pub coeffs: Vec<u8>
}

impl Polynomial {
    pub fn new(coeffs: &[u8]) -> Self {
        let c = coeffs.to_vec();
        Polynomial{
            coeffs: c,
        }
    }
    // evaluate p(z) using horner's algorithm
    // empty polynomial means fucking nothing. coeffs have to be represented as 0s.
    pub fn eval(&self, z: u8) -> Option<u8>{
        let coeffs = &self.coeffs;
        if coeffs.is_empty() {
            None
        } else if coeffs.iter().all(|c| *c==0){
            Some(0)
        } else {
            let mut t = coeffs[0];
            for i in 1..coeffs.len() {
                t = gf256::GF256::add(gf256::GF256::mul(t,z), coeffs[i]);
            }
            Some(t)
        }
    }

    pub fn degree(&self) -> Option<u32> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some((self.coeffs.len()-1) as u32)
        }
    }

    
}


//fuckass test written for obvious fuckass cases.. im never debugging shit.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_evaluation() {
        let coeffs1 = vec![4, 8, 1];
        let coeffs2 = vec![0, 0, 0, 0];
        let coeffs3: Vec<u8> = vec![];
        let poly1 = Polynomial {
            coeffs: coeffs1,
        };
        let poly2 = Polynomial {
            coeffs: coeffs2,
        };
        let poly3 = Polynomial {
            coeffs: coeffs3,
        };

        assert_eq!(Polynomial::eval(&poly1, 1), Some(13));
        assert_eq!(Polynomial::eval(&poly2, 4), Some(0));
        assert_eq!(Polynomial::eval(&poly3, 2), None);
    }
}



