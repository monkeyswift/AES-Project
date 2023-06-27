#[derive(Clone, Debug)]
pub struct Polynomial {
    pub poly: Vec<u8>
}

impl Polynomial {

    fn iter(&self) -> std::slice::Iter<u8> {
        self.poly.iter()
    }

    pub fn aes_modulo(mut self) -> Polynomial {

        let irr_poly = Polynomial {poly: vec![8, 4, 3, 1, 0]};

        while self.poly[0] >= 8 {
            let div_of_first_terms = self.poly[0] - irr_poly.poly[0]; // storing the quotient of dividing the first terms of both polynomials as res
            let subtrahend: Polynomial = Polynomial {poly: irr_poly.iter().map(|term| div_of_first_terms + *term).collect()}; // creating the polynomial that needs to be subtracted from self.
            self = self - subtrahend;
        }
        self
    }

}

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;
fn sub(mut self, mut subtrahend: Polynomial) -> Polynomial {

    self.poly.append(&mut subtrahend.poly);

    let mut new_self = vec![];

    for term1 in self.iter() {
        let mut count = 0;

        for term2 in self.iter() {
        
            if *term1 == *term2 {
                count += 1;
            }
        }
            if count % 2 != 0 {
                new_self.push(*term1)  
            }  
    }
    new_self.sort();
    new_self = new_self.into_iter().rev().collect();
    new_self.dedup();
            Polynomial {poly: new_self}
        }
}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(mut self, poly2: Polynomial) -> Polynomial {
        
        let poly = 
        self.iter().flat_map(|degree1| {
            poly2.iter().map(move|degree2| -> u8 {
                
                *degree1  + *degree2

            })
        }).collect::<Vec<u8>>(); // from here on out I need to remove like terms if they appear more than twice. After that I also need to fix the order.
    //The block below does addition in the field gf(2) to simplify the polynomial product. 
        self.poly.clear();
        for term1 in poly.iter() {
            let mut count = 0;

            for term2 in poly.iter() {
            
                if term1 == term2 {
                    count += 1;
                }
            }
                if count % 2 != 0 {
                    self.poly.push(*term1)  
                }  
        }
        self.poly.sort();
        self.poly = self.poly.into_iter().rev().collect();
        self.poly.dedup();
        
        self
    }
}

pub fn binary_to_polynomials(columns: Vec<Vec<u8>>) -> Vec<Vec<Polynomial>> {

    columns.into_iter().map(|column| {
        column.into_iter().map(|byte| -> Polynomial {
            let mut temp_vec = Vec::new();
            for n in (0..8).rev() {
                let bit = (byte >> n) & 1;
                if bit == 1 {
                    temp_vec.push(n)
                }
            }
            Polynomial {poly: (temp_vec)}
        }).collect()
    }).collect()

}

pub fn polynomials_to_binary(polynomials: Vec<Vec<Polynomial>>) -> Vec<Vec<u8>> {
    polynomials.into_iter().map(|polynomial_vec| {
        polynomial_vec.into_iter().map(|polynomial| {
            let mut byte: u8 = 0;
            for n in polynomial.poly {
                byte |= 1 << n;
            }
            byte
        }).collect()
    }).collect()
}
