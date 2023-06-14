#[derive(Clone, Debug)]
pub struct Polynomial {
    pub poly: Vec<u8>
}
//remember that addition is just Xor cause of the finite field.
impl Polynomial {

    fn iter(&self) -> std::slice::Iter<u8> {
        self.poly.iter()
    }

    pub fn aes_modulo(mut self) -> Polynomial {

        let irr_poly = Polynomial {poly: vec![8, 4, 3, 1, 0]};

        while self.poly[0] >= 8 {
            println!("round executed\n");
            let div_of_first_terms = self.poly[0] - irr_poly.poly[0]; // storing the quotient of dividing the first terms of both polynomials as res
            println!("the quotient of dividing the first two terms is: {}", div_of_first_terms);
            let subtrahend: Polynomial = Polynomial {poly: irr_poly.iter().map(|term| div_of_first_terms + *term).collect()}; // creating the polynomial that needs to be subtracted from self.
            println!("self before subtraction: {:?}, subtrahend before subtraction: {:?}", self, subtrahend);
            self = self - subtrahend;
            println!("self after subtraction: {:?}", self);
        }
        self
    }

}
// writing it out just helps me. So I currently have a problem where I need to include terms from the subtrahend if they aren't matched by anything.
// if, after iterating througout all of self and comparing each element to the subtrahend term everything is included, the subtrahend term should be added.  

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;
fn sub(mut self, subtrahend: Polynomial) -> Polynomial {

    let mut temp_self = self.poly.clone();
        
    let mut subtrahend_remains = vec![];
        
    let mut counter = 0;
        
        subtrahend.iter().enumerate().for_each(|(index1, sub_term)| {
                temp_self.clear();
                self.clone().iter().enumerate().for_each(|(index2, self_term)|{
                        
                if *sub_term != *self_term {
                    temp_self.push(*self_term);
                    counter += 1;
                } else {
                }
                });
                    if self.poly.len() == counter {

                        subtrahend_remains.push(*sub_term)
                    }
                    counter = 0;
                    
                    self = Polynomial {poly: temp_self.clone()};
            });
            
            subtrahend_remains.iter().for_each(|remains_term| {
            for (index2, self_term) in temp_self.iter().enumerate().rev() {
                if *remains_term < *self_term {
                    self.poly.insert(index2 + 1 + counter, *remains_term);
                    counter += 1;
                    break;
                }
            }
            });
            self
        }
}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(mut self, poly2: Polynomial) -> Polynomial {
        
        let poly = 
        self.iter().flat_map(|degree1| {
            poly2.iter().map(move|degree2| -> u8 {
                
                *degree1 + *degree2

            })
        }).collect::<Vec<u8>>(); // from here on out I need to remove like terms if they appear more than twice. After that I also need to fix the order.
    println!("{:?}", poly);
     let mut simplified_poly = vec![255];

     let mut gremlin = vec![];

        for term1 in poly.iter() {

            let mut count = 1;

            for term2 in poly.iter() {
                if term1 == term2 {
                    count += 1
                }
            }
            if count % 2 == 0 {
                if simplified_poly[simplified_poly.len() - 1] > *term1 {
                    simplified_poly.push(*term1)
                } else {
                    gremlin.push(*term1)
                }
            }   
        }
        let mut counter = 0;
        self.poly.clear();
        gremlin.iter().for_each(|gremlin_term| {
            for (index2, self_term) in simplified_poly.iter().enumerate().rev() {
                if *gremlin_term < *self_term {
                    self.poly.insert(index2 + 1 + counter, *gremlin_term);
                    counter += 1;
                    break;
                }
            }
            });

        Polynomial {poly: simplified_poly}
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
