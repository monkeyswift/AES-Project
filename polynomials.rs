#[derive(Clone, Debug)]
pub struct Polynomial{
    coeffs: Vec<u8>
}
fn iter(&self) -> std::slice::Iter<u8> {
        self.coeffs.iter()
    }
}

//first I need to pad out the polynomial
//

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(poly1: self, poly2: Polynomial) -> Polynomial {
        let mut needs_simplification = poly1.iter().map(|poly1_term: &u8| {
            poly2.iter().map(|poly2_term: &u8| {
                poly1_term * poly2_term
            })
        }).collect();


    }
}

// function below is problematic. It takes the direct le byte representation and turns it into a vector that is stored in the polynomial. The issue here is with
// how binary is represented in polynomial form. Here the rightmost end of the binary corresponds to the coefficient of the term with the highest degree in the polynomial
// this isn't how this is conventionally done. So you either need to flip the bit order so that this isn't the case or you need to change how polynomials are represented
// in the program.

pub fn polynomial_conversion(columns: Vec<Vec<u8>>) -> Vec<Vec<Polynomial>> {

    let mut columns_as_polynomials: Vec<Vec<Polynomial>> = Vec::new();

    let mut temp_vec: Vec<Polynomial> = Vec::new();

    for column in columns.into_iter() {
        column.into_iter().enumerate().for_each(|(index, byte)| {
            let r_index = index +1;
            if r_index % 4 != 0 {
                temp_vec.push(Polynomial {coeffs: byte.to_le_bytes().to_vec()});
            } else {
                temp_vec.push(Polynomial {coeffs: byte.to_ne_bytes().to_vec()});
                columns_as_polynomials.push(temp_vec.clone());
                temp_vec.clear();
            }
        });
    }

columns_as_polynomials

}
