///Fast two sum
pub fn two_sum(a: f64, b: f64) -> Vec<f64> {
    let x  = a + b;
    let bv = x - a;
    let av = x - bv;
    let br = b - bv;
    let ar = a - av;
    vec!(ar + br, x)
}



#[cfg(test)]
mod two_sum_test {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(1e+64, 1.0) , [1.0, 1e+64]);
        assert_eq!(two_sum(1., 1.) , [0., 2.]);
        assert_eq!(two_sum(0., -1415.) , [0., -1415.]);
        assert_eq!(two_sum(1e-64, 1e64) , [1e-64, 1e64]);
        assert_eq!(two_sum(0., 0.) , [0., 0.]);
        assert_eq!(two_sum(9e15 + 1., 9e15) , [1., 18000000000000000.]);
    }
}

