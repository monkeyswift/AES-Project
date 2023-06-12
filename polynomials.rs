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
// writing it out just helps me. So I currently have a problem where I need to include terms from the subtrahend if they aren't matched by anything.
// if, after iterating througout all of self and comparing each element to the subtrahend term everything is included, the subtrahend term should be added.  

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;
    fn sub(mut self, subtrahend: Polynomial) -> Polynomial {

        let mut temp_self = self.poly.clone();

        let mut subtrahend_remains = vec![];

        let mut counter = 0;

        //let mut push_count = 0;

        //let mut self_count = self.len();

        subtrahend.iter().enumerate().for_each(|(index1, sub_term)| {
            println!("sub_term {} is in use", index1 + 1);
            println!("you are on round {} \n", index1 + 1);
            temp_self.clear();
            self.clone().iter().enumerate().for_each(|(index2, self_term)|{
                
            if *sub_term != *self_term {
                temp_self.push(*self_term);
                counter += 1;
            } else {
                println!("excluded term {} from new self", index2 + 1);
            }
            println!("index of self: {}, length of self {} \n", index2, self.poly.len() - 1);
            if index2 == self.poly.len() - 1 {
                println!("end of iterator has been reached");
                if index2 + 1 == counter {
                    println!("added term to subtrahend");
                    subtrahend_remains.push(*sub_term)
                }
                counter = 0;
                
                println!("updated self from inner loop");
                self = Polynomial {poly: temp_self.clone()}; 
                
            }
            
            })
        });
        println!("{:?}", subtrahend_remains);
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


//Everything below this point is in the scrapyard
//pub fn evaluate(&self, x: u8) -> u8 {
    //let mut result = 0;
    //for &coeff in self.poly.iter().rev() {
        //result = result * x + coeff;
    //}
    //result
//}
//while self.poly[0] >= divisor.poly[0] {
    //let mut new_self = Vec::new();
    //for (index ,dividend_term_degree) in self.iter().enumerate() {
        //for subtractor_term_degree in subtractor.iter() {
            //if *dividend_term_degree != *subtractor_term_degree {
                //new_dividend.push(*dividend_term_degree);
                //let mut temp_self = self.poly.clone();
                //temp_self.remove(index);
                //if subtractor.iter().len() == *subtractor_term_degree as usize{
                    //new_self.extend(temp_self);
                //}
            //}
        //}
    //}
    //self.poly = new_self;
//}



//let mut new_self = Vec::new();
            //for (index ,dividend_term_degree) in self.iter().enumerate() {
                //for subtractor_term_degree in subtractor.iter() {
                    //if *dividend_term_degree != *subtractor_term_degree {
                        //new_dividend.push(*dividend_term_degree);//
                        //let mut temp_self = self.poly.clone();
                        //temp_self.remove(index);
                        //if subtractor.iter().len() == *subtractor_term_degree as usize{
                            //new_self.extend(temp_self);
                        //}
                    //}
                //}
            //}
