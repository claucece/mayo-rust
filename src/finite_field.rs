// Methods that define arithmetic over GF(16), with irreducible polynomial of degree 4 over GF(2).
// Concretely, f(x) = x^4 + x + 1 is used. 
use std::u8;

use crate::constants::{K, M, N, O, V};


// Negation in GF(16) of any element is the element itself because a is it's own additive inverse (where 0 is the additive identity).
// Hence, -a = a in binary fields (GF(2^n)).  
pub fn neg(x: u8) -> u8 {
    return x; // Negation in GF(2^n) has no effect as a + a = 0.
}

// GF(16) addition is equivalent to XOR because we do bitwise addition modulo 2 (no carry)
pub fn add(x: u8, y: u8) -> u8 {
    return x ^ y;
}

// GF(16) subtraction is equivalent to XOR because we do bitwise subtraction modulo 2 (no carry)
pub fn sub(x: u8, y: u8) -> u8 {
    return x ^ y;
}

// GF(16) multiplication is equivalent to multiplying the polynomials and then reducing modulo the irreducible polynomial. 
pub fn mul(x: u8, y: u8) -> u8 {

    // Carryless multiplication of polynomials in GF(2^4)
    let mut res: u8;
    res =  (x & 1)*y; // Multiply by x^0
    res ^= (x & 2)*y; // Multiply by x^1
    res ^= (x & 4)*y; // Multiply by x^2
    res ^= (x & 8)*y; // Multiply by x^3

    // Reduce modulo by the irreducible polynomial x^4 + x + 1 
    let first_4_bits: u8 = res & 0xf0; // First 4 bits of res (x^7 to x^4. Notice, the first bit is always 0, cause we can't get more than x^6)
    let overflow_bits: u8 = (first_4_bits >> 4) ^ (first_4_bits >> 3); // Replace x^4 with x + 1 as x^4 (e.g. 16) = x + 1 (under the irreducible polynomial). Notice, + is XOR in binary fields.
    let res : u8 = (res ^ overflow_bits) & 0x0f; // XOR res with the mod reduction of the overflow bits. Then remove first 4 bits from res.
    return res;
}

// From Fermat's little theorem, we know that an element x in a finite field F satisfies x^{p^{n}-1} = 1,
// where p is the characteristic of F and n is the degree of the extension. From this we can deduce that x^{14} * x = x^{-1} * x = 1.
// E.g. x^14 = x^-1 (the multiplicative inverse of x)      
pub fn inv(x: u8) -> u8{

    // u8 table[16] = {0, 1, 9, 14, 13, 11, 7, 6, 15, 2, 12, 5,
    // 10, 4, 3, 8}; return table[a & 15];

    // Calculate multiplicative inverse of x by exponentiation by squaring (x^14 = x^-1) 
    let x2: u8 = mul(x, x);
    let x4: u8 = mul(x2, x2);
    let x6: u8 = mul(x2, x4);
    let x8: u8 = mul(x4, x4);
    let x14: u8 = mul(x8, x6);

    return x14;
}


// GF(16) division is equivalent to multiplying the dividend by the multiplicative inverse of the divisor.
pub fn div(x: u8, y: u8) -> u8 {
    return mul(x, inv(y));
}




#[macro_export]
macro_rules! matrix_add {
    ($a:expr, $b:expr, $rows:expr, $cols:expr) => {{
        for i in 0..$rows {
            for j in 0..$cols {
                $a[i][j] = add($a[i][j], $b[i][j]); // Perform addition directly on matrix a
            }
        }
        $a // Return the modified matrix a as the result
    }};
}




#[macro_export]
macro_rules! matrix_mul {
    ($a:expr, $rows_a:expr, $cols_a:expr, $b:expr, $cols_b:expr) => {{
        let mut result = [[0u8; $cols_b]; $rows_a];

        for i in 0..$rows_a {
            for j in 0..$cols_b {
                for k in 0..$cols_a {
                    // Take the dot product of the i-th row of A and the j-th column of B
                    result[i][j] = add(result[i][j], mul($a[i][k], $b[k][j]));
                }
            }
        }
        result
    }};
}


pub fn matrix_mul_v_l(a: [u8; V], b: [[u8; O]; V]) -> [u8 ; O] {

    let mut result = [0; O];

    for j in 0..V {
        for k in 0..O {
            // Take the dot product of the i-th row of A and the j-th column of B
            result[k] = add(result[k], mul(a[j], b[j][k])); 

        }
    }
    return result
}

pub fn matrix_mul_s_trans_big_p(s: [u8; N], big_p: [[u8; N]; N]) -> [u8 ; N] {

    let mut result = [0; N];

    for j in 0..N {
        for k in 0..N {
            // Take the dot product of the i-th row of A and the j-th column of B
            result[k] = add(result[k], mul(s[j], big_p[j][k])); 

        }
    }
    return result
}

pub fn matrix_mul_v_p1(v: [u8; V], p1: [[u8; V]; V]) -> [u8 ; V] {

    let mut result = [0; V];

    for j in 0..V {
        for k in 0..V {
            // Take the dot product of the i-th row of A and the j-th column of B
            result[j] = add(result[j], mul(v[k], p1[k][j])); 

        }
    }
    return result
}

pub fn array_mul_s_p(s: [u8; N], p: [u8; N]) -> u8 {

    let mut result = 0;

    for i in 0..N {
            // Take the dot product of the i-th row of A and the j-th column of B
            result = add(result, mul(s[i], p[i])); 
    }
    return result
}



// Vector-matrix multiplication over GF(16).
// Returns a vector of size equal to the number of columns in the matrix.
pub fn vector_matrix_mul(vec: &Vec<u8>, matrix: &Vec<Vec<u8>>) -> Vec<u8> {
    
    assert_eq!(vec.len(), matrix.len(), "Length of vector must equal number of rows in matrix");

    let rows_matrix = matrix.len();
    let cols_matrix = matrix[0].len();

    let mut result = vec![0; cols_matrix]; // 1 x cols_matrix vector

    for j in 0..cols_matrix {
        for i in 0..rows_matrix {
            // Multiply each element of the vector by the corresponding element in the matrix column and sum the results
            result[j] = add(result[j], mul(vec[i], matrix[i][j])); 
        }
    }
    return result;
}


// Matrix-vector multiplication over GF(16)
// Returns a vector of size equal to the number of rows in the matrix.
pub fn matrix_vector_mul(matrix: &Vec<Vec<u8>>, vec: &Vec<u8>) -> Vec<u8> {
    assert_eq!(matrix[0].len(), vec.len(), "Number of columns in matrix must equal length of vector");

    let rows_matrix = matrix.len();
    let cols_matrix = matrix[0].len();

    let mut result = vec![0; rows_matrix]; // rows_matrix x 1 vector

    for i in 0..rows_matrix {
        for k in 0..cols_matrix {
            // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
            result[i] = add(result[i], mul(matrix[i][k], vec[k])); 
        }
    }

    return result;
}

pub fn a_mul_r(matrix: [[u8; K*O]; M], array: [u8 ; K*O]) -> [u8 ; M] {
    let mut result = [0; M]; // rows_matrix x 1 vector


    for i in 0..M {
        for j in 0..K*O {
            // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
            result[i] = add(result[i], mul(matrix[i][j], array[j])); 
        }
    }
    return result;
}


pub fn o_matrix_x_idx_mul(matrix: [[u8; O]; V], array: &[u8]) -> [u8 ; V] {
    let mut result = [0u8 ; V]; // V x 1 vector

    for i in 0..V {
        for j in 0..O {
            // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
            result[i] = add(result[i], mul(matrix[i][j], array[j])); 
        }
    }

    return result;
}



pub fn p1_matrix_v_mul(p1: [u8; V], v: [u8 ; V]) -> u8 {
    let mut result = 0; // rows_matrix x 1 vector


    for i in 0..V {
        // Multiply each element of the i-th row of the matrix by the corresponding element in the vector and sum the results
        result = add(result, mul(p1[i], v[i])); 
    }

    return result;
}




#[macro_export]
macro_rules! transpose_matrix_array {
    ($matrix:expr, $rows:expr, $cols:expr) => {{
        // Initialize the transposed matrix with zeroes. The dimensions are swapped.
        let mut transposed = [[0u8; $rows]; $cols];

        for i in 0..$rows {
            for j in 0..$cols {
                transposed[j][i] = $matrix[i][j]; // Swap elements
            }
        }
        transposed
    }};
}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        // Negatrion is defined as the additive inverse of a number. 
        // E.g. how much we need to add to a number to get 0. (always the number itself in binary fields)
        assert_eq!(neg(0x0), 0x0); // 0 is its own negation
        assert_eq!(neg(0x1), 0x1); // 1 is its own negation 
        assert_eq!(neg(0xf), 0xf); // 0xf is its own negation
        assert_eq!(neg(0xe), 0xe); // 0xe is its own negation

    }

    #[test]
    fn test_add() {
        assert_eq!(add(0x0, 0x0), 0x0); // 0 is the additive identity
        assert_eq!(add(0x1, 0x1), 0x0); // 1 is its own additive inverse
        assert_eq!(add(0x1, 0x2), 0x3); // 1 + 2 = 3
        assert_eq!(add(0x3, 0x1), 0x2); // 3 + 1 = 2
        assert_eq!(add(0x6, 0x6), 0x0); // 6 is its own additive inverse
    }

    #[test]
    fn test_sub() {
        // Subtraction is the same as addition in GF(16)
        assert_eq!(sub(0x0, 0x0), 0x0); // 0 is the additive identity
        assert_eq!(sub(0x3, 0x1), 0x2); // (x + 1) - 1 = x
        assert_eq!(sub(0x1, 0x2), 0x3); // 1 - x = x + 1
        assert_eq!(sub(0x6, 0xf), 0x9); // x^2 + x - (x^3 + x^2 + x + 1) = x^3 + 1
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(0x0, 0x0), 0x0); // 0 * 0 = 0
        assert_eq!(mul(0x1, 0x1), 0x1); // 1 * 1 = 1
        assert_eq!(mul(0x2, 0x2), 0x4); // x * x = x^2 = 4
        assert_eq!(mul(0x3, 0x3), 0x5); // (x + 1) * (x + 1) = x^2 + 2x + 1 = x^2 + 1 (as modulo 2 eats the 2x - no modular reduction needed)

        assert_eq!(mul(0xC, 0x3), 0x7); 
        // (x^3 + x^2) * (x + 1) = x^4 + 2x^3 + x^2 
        // = x^4 + x^2 (Term-wise modulo 2 reduction)
        // x^4 + x^2 + (x^4 + x + 1) = x^2 + x + 1 (By doing modular reduction on x^4 + x^2 with f(x) = x^4 + x + 1 - then modulo 2 term-wise)

        assert_eq!(mul(0xC, 0x7), 0x2); 
        // (x^3 + x^2) * (x^2 + x + 1) = x^5 + 2x^4 + 2x^3 + x^2 = x^5 + x^2 (Term-wise modulo 2 reduction)
        // x^5 + x^2 + (x^5 + x^2 + x) = 2x^5 + 2x^2 + x (By doing modular reduction on x^5 + x^2 with x * f(x) = x^5 + x^2 + x)
        // = x  (modulo 2 term-wise)

        assert_eq!(mul(0xf, 0xf), 0xa); 
        // (x^3 + x^2 + x + 1) * (x^3 + x^2 + x + 1) = x^6 + 2x^5 + 3x^4 + 4x^3 + 3x^2 + x + 1
        // = x^6 + x^4 + x^2 + 1 (Term-wise modulo 2 reduction)
        // x^6 + x^4 + x^3 + x^2 + 1 + (x^6 + x^3 + x^2) = 2x^6 + x^4 + 2x^3 + 2x^2 + 1 (By doing modular reduction on x^6 + x^4 + x^3 + x^2 + 1 with x^2 * f(x) = x^6 + x^3 + x^2)
        // = x^4 + x^3 + 1 (modulo 2 term-wise)
        // x^4 + x^3 + 1 + (x^4 + x + 1) = 2x^4 + x^3 + x + 2 (By doing modular reduction on x^4 + x^3 + 1 with f(x) = x^4 + x + 1)
        // = x^3 + x  (modulo 2 term-wise)
    }

    #[test]
    fn test_inv() {
        assert_eq!(inv(0x0), 0x0); // 0 acts as its own inverse, but theorethically it's undefined
        assert_eq!(inv(0x1), 0x1); // 1 is its own inverse
        // For non-trivial inverses, mul(x, inv(x)) = 1
        assert_eq!(inv(0x2), 0x9); // x's inverse is x^3 + 1 
        assert_eq!(inv(0x3), 0xe); // (x + 1)'s inverse is x^3 + x^2 + x
        assert_eq!(inv(0x4), 0xd); // x^2's inverse is x^3 + x^2 + 1
        assert_eq!(inv(0x5), 0xb); // (x^2 + 1)'s inverse is x^3 + x + 1
        assert_eq!(inv(0x6), 0x7); // (x^2 + x)'s inverse is x^2 + x + 1
        assert_eq!(inv(0x8), 0xf); // x^3's inverse is x^3 + x^2 + x + 1
    }


}