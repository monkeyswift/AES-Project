#[derive(Clone, Debug)]
pub struct Polynomial {
    pub coeffs: Vec<u8>
}

//remember that addition is just Xor cause of the finite field.

impl Polynomial {

    pub fn evaluate(&self, x: u8) -> u8 {
        let mut result = 0;
        for &coeff in self.coeffs.iter().rev() {
            result = result * x + coeff;
        }
        result
    }

    fn iter(&self) -> std::slice::Iter<u8> {
        self.coeffs.iter()
    }

    fn into_iter(self) -> std::vec::IntoIter<u8> {
        self.coeffs.into_iter()
    }

    //fn modulo(self, dividend: Polynomial) -> Polynomial {
        
    //}
}

//first I need to pad out the polynomial
//I will pad out the polynomial and I will use the index of the element in the vector as the degree of the polynomial. 

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, poly2: Polynomial) -> Polynomial {
        
        //let mut needs_simplification = self.iter().map(|poly1_term: &u8| {
            //poly2.iter().map(|poly2_term: &u8| {
                //poly1_term * poly2_term
            //})
        //}).collect();
        Polynomial {coeffs: vec![1]}

    }
}

//impl std::ops::Add for Polynomial {
    //type Output = Polynomial;
    //fn add(self, poly2: Polynomial) -> Polynomial {
        
    //}
//}

pub fn polynomial_conversion(columns: Vec<Vec<u8>>) -> Vec<Vec<Polynomial>> {

    columns.into_iter().map(|column| {
        column.into_iter().map(|byte| -> Polynomial {
            let mut temp_vec = Vec::new();
            for n in (0..8).rev() {
               temp_vec.push((byte >> n) & 1);
            }
            Polynomial {coeffs: temp_vec}
        }).collect()
    }).collect()

}
