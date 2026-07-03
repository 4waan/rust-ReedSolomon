use crate::poly::Polynomial;
/*
 * Look at ethereum DAS (Data Availability Sampling) client.. It wouldnt be realistic for it to 
 * blindly trust every chunk it receives to be correct. There obviously cannot be ignorance towards 
 * malicious data so polynomial commitment systems re adapted to let a client verify a chunk is genuine
 * before marking "missing" and considering that missing is literally the only failure mode.
 * 
 * 
 * lets look at snother problem.. take a polynomial p of degree k-1, and we have n transferred values
 * t of those n are corrupted so we receive n-t genuine values matching p.
 * Now take a polynomial q!=p with degree <= (k-1) such that q matches the received values on k points too
 * now q also looks like a valid explanation for our recieved data.
 * Its possible to do that.. p(a)=A p(b)=B and p(c)=C then we can definitely construct a polynomial q(x)
 * that matches k such values. 
 * Also we can suppose that if p is wrong on t values then q is also wrong on t values.
 * (This brings us at a huge problem.. q works on the decoder side but it literally gives out another message
 * than expected).
 * 
 * then we can derive: 
 * if p is correct at n-t values and q is correct at n-t values and both are wrong at t points..
 * and we re at an exclusion= n.
 * then the minimum number of points where they overlap is
 * overlap>= correct_p + correct_q - exclusion {|A ∩ B| ≥ |A| + |B| - n}
 * overlap>= n-t +n-t -n
 * overlap>= n-2t
 * 
 * The fact to be noted quite well is:
 * Two distinct degree k-1 polynomials can intersect at atmost k-1 points
 * but we need p and q to remain distinct..
 * so the maximum overlap points should be limited at k-1
 * --> [n-2t<=k-1]
 * --> t>=(n-k+1)/2
 * 
 * Here we arrive at a point to realise that for such a q to exist that would
 * literally confuse the decoder: t>=(n-k+1)/2
 * So for correct and unambiguous decoding we need
 * t<(n-k+1)/2
 * but since t is an integer.. 
 * [t<(n-k)/2]
 */

 fn encode(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoded: Vec<u8> = Vec::new();
    let n = 2*(data.len()); // n=2k for 50% error tolerance
    if data.len()== 0 {
        return Err("".to_string())
    } else {
        let poly = Polynomial::new(data);
        for i in 1..n+1 {
            encoded.push(
                poly.eval(i).unwrap()
            );
        }
    }
    Ok(encoded)
 }