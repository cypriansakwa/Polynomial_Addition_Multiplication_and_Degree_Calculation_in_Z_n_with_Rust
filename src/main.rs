fn main() {
    // Define polynomials P(x) = 5x^2 - 4x + 2 and Q(x) = x^3 - 2x^2 + 5 in Z_6[x]
    let p = vec![2, -4, 5];  // Coefficients of P(x): 2 + (-4)x + 5x^2
    let q = vec![5, 0, -2, 1]; // Coefficients of Q(x): 5 + (-2)x^2 + x^3
    //$f,g\in \mathbb{Z}_{17}[x]$ with $f(x) = 3x^3-15x^2+12x - 13$ and $g(x) = -7x^3-6x^2 + 10$
    // Perform addition and multiplication modulo 6
    let sum = add_polynomials_mod(&p, &q, 6);
    let product = multiply_polynomials_mod(&p, &q, 6);
    
    // Calculate degrees
    let deg_sum = degree(&sum);
    let deg_product = degree(&product);
    
    // Print the results
    println!("P(x) + Q(x) mod 6 = {:?}", sum);
    println!("P(x) * Q(x) mod 6 = {:?}", product);
    println!("deg(P + Q) = {}", deg_sum);
    println!("deg(P * Q) = {}", deg_product);
}

// Function to add two polynomials in Z_n[x]
fn add_polynomials_mod(p: &Vec<i32>, q: &Vec<i32>, n: i32) -> Vec<i32> {
    let max_len = std::cmp::max(p.len(), q.len());
    let mut result = vec![0; max_len];

    for i in 0..p.len() {
        result[i] = (result[i] + p[i]) % n;
        if result[i] < 0 {
            result[i] += n; // Ensure non-negative values in Z_n
        }
    }

    for i in 0..q.len() {
        result[i] = (result[i] + q[i]) % n;
        if result[i] < 0 {
            result[i] += n;
        }
    }

    result
}

// Function to multiply two polynomials in Z_n[x]
fn multiply_polynomials_mod(p: &Vec<i32>, q: &Vec<i32>, n: i32) -> Vec<i32> {
    let mut result = vec![0; p.len() + q.len() - 1];

    for (i, &p_coeff) in p.iter().enumerate() {
        for (j, &q_coeff) in q.iter().enumerate() {
            result[i + j] = (result[i + j] + (p_coeff * q_coeff) % n) % n;
            if result[i + j] < 0 {
                result[i + j] += n;
            }
        }
    }

    result
}

// Function to find the degree of a polynomial
fn degree(poly: &Vec<i32>) -> usize {
    // The degree is the index of the last non-zero coefficient
    for i in (0..poly.len()).rev() {
        if poly[i] != 0 {
            return i;
        }
    }
    0 // Degree of zero polynomial is conventionally 0
}


