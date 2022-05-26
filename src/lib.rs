use std::io;
pub trait Math {
    fn math_x(&self) -> f64;
    fn print_x(&self) {
       println!("result: {:?}", self.math_x())
   }
}
pub struct Sqare (f64);
impl Math for Sqare {
    fn math_x(&self) -> f64 {
        &self.0 * 4.0
    }
}


fn read() -> f64 {
    loop {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let x: Result<f64, _> = guess.trim().parse();
    match x {
        Ok(amount) => return amount,
        Err(_) => println!("Please type a number!"),
    }
}
}
impl Sqare {
    pub fn read_square() -> Self {
        println!("Type side size of sqare");
        let x = read();
        Self(x)
    }

}
pub struct Rectangle (f64, f64);
impl Rectangle {
    pub fn read_rectangle() -> Self {
        println!("Type side size a of rectangle");
        let x = read();
        println!("Type side size b of rectangle");
        let y = read();
        Self(x, y)
    }
}
impl Math for Rectangle {
 fn math_x(&self) -> f64 {
        (self.0 + self.1) * 2.0
   }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_sqare() {
        let x = Sqare (1.0);
        assert_eq!(x.math_x(), 4.0);
    }
    
    #[test]
    fn math_rectangle() ->Result<(), String>  {
        let x = Rectangle (1.0, 1.0);

        match x.math_x() {
            result if result == 4.0 => Ok(()),
            _ => Err("Must be 4.0".to_string())
        }

        


        // if x.math_x() == 4.0 {
        //     Ok(())           
        // } else {
        //     Err("Must be 4".to_owned())
        // }
    }

}

