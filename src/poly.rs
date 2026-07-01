

/*
 * To evaluate p(x) at some point z, you compute:
 * p(z) = c₀ + c₁z + c₂z² + c₃z³ + ...
 * There's a nice algorithm for this called Horner's method:
 * p(z) = c₀ + z(c₁ + z(c₂ + z(c₃ + ...)))
 * this is the direction we build our coeffs vector in to create a polynomial.
*/

// evaluate p(z)
fn eval(coeffs: &[u8], z: u8) -> u8 {
    
    todo!()
}

