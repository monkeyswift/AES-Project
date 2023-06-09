use crate::polynomials::NumType::{Degree, Coefficient};

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub poly: Vec<(NumType, NumType)>
}
#[derive(Clone, Debug, PartialEq)]
pub enum NumType {
    Degree (u8),
    Coefficient (u8),
}

//remember that addition is just Xor cause of the finite field.

impl Polynomial {

    fn iter(&self) -> std::slice::Iter<(NumType, NumType)> {
        self.poly.iter()
    }

    fn into_iter(self) -> std::vec::IntoIter<(NumType, NumType)> {
        self.poly.into_iter()
    }

    fn modulo(self, divisor: Polynomial) -> Polynomial {
        let res = self.poly[0] / divisor.poly[0];
        let new_divisor: Vec<u8> = divisor.iter().map(|term| &res * term).collect();
        let new_dividend = self.iter().zip(new_divisor.iter()).map(|(s_term, dterm)| );

        let temp_return = Polynomial {poly: vec![(Degree(1), Coefficient(1)), (Degree(0), Coefficient(1))]};
        temp_return
    }
}

//first I need to pad out the polynomial
//I will pad out the polynomial and I will use the index of the element in the vector as the degree of the polynomial.
// when you multiply two terms, you take their index and 

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, poly2: Polynomial) -> Polynomial {
        
        let mut unpadded_poly = self.iter().filter(|(_pd, term)| *term == Coefficient(1)).flat_map(|(predegree1, _poly1_term)| {
            poly2.iter().filter(|(pd, term)| *term == Coefficient(1)).map(move|(predegree2, _poly2_term)| -> () {
                
                
                
                
                
            })
        }).collect::<Vec<((NumType, NumType))>>();

        //before I clean up and implement addition between the remaining terms I need to decide on how I want to do modulo, as I'm not sure if I will need to pad
        //out the resulting polynomial. 

        Polynomial {poly: vec![(Degree(0), Coefficient(0))]}

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
                let bit = ((byte >> n) & 1);
                if bit == 1 {
                    temp_vec.push((Degree(n), Coefficient(bit)))
                }
            }
            Polynomial {poly: (temp_vec)}
        }).collect()
    }).collect()

}


//Everything below this point is in the scrapyard
//pub fn evaluate(&self, x: u8) -> u8 {
    //let mut result = 0;
    //for &coeff in self.poly.iter().rev() {
        //result = result * x + coeff;
    //}
    //result
//}
