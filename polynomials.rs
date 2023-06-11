#[derive(Clone, Debug)]

//Each element of the vector represents the degree of the x. I realized that I don't need to store coefficient values because coefficients are only ever 1 or 0, so if the 
//degree exists at all it implies that the coefficient is 1. 
pub struct Polynomial {
    pub poly: Vec<u8>
}
//remember that addition is just Xor cause of the finite field.
impl Polynomial {

    fn iter(&self) -> std::slice::Iter<u8> {
        self.poly.iter()
    }

    fn into_iter(self) -> std::vec::IntoIter<u8> {
        self.poly.into_iter()
    }

    pub fn aes_modulo(mut self, mut divisor: Polynomial) -> Polynomial {
        let res = self.poly[0] / divisor.poly[0]; // storing the quotient of dividing the first terms of both polynomials as res
        let subtrahend: Vec<u8> = divisor.iter().map(|term| &res * term).collect(); // creating the polynomial that needs to be subtracted from self. 

        Polynomial {poly: vec![1]}
    }

}
// writing it out just helps me. So I currently have a problem where I need to include terms from the subtrahend if they aren't matched by anything. my logic so 

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;
    fn sub(mut self, subtrahend: Polynomial) -> Polynomial {

        let mut temp_self = self.poly.clone();

        subtrahend.iter().enumerate().for_each(|(index1, sub_term)| {
            self = Polynomial{poly: temp_self.clone()};
            temp_self.clear();
            self.clone().iter().enumerate().for_each(|(index2, self_term)|{
            if *sub_term != *self_term {

                if index2 == self.poly.len() - 1 {
                    temp_self.push(*sub_term)
            }
                temp_self.push(*self_term);
            }
            if index1 == subtrahend.poly.len() - 1 {
                self = Polynomial {poly: temp_self.clone()};
            } 
            })
        });
        self
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, poly2: Polynomial) -> Polynomial {
        
        let mut poly = 
        self.iter().flat_map(|degree1| {
            poly2.iter().map(move|degree2| -> u8 {
                
                *degree1 + *degree2

            })
        }).collect::<Vec<u8>>();

        //before I clean up and implement addition between the remaining terms I need to decide on how I want to do modulo, as I'm not sure if I will need to pad
        //out the resulting polynomial. 

        Polynomial {poly: vec![0]}

    }
}

pub fn polynomial_conversion(columns: Vec<Vec<u8>>) -> Vec<Vec<Polynomial>> {

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
