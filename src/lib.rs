use std::{io, f64::consts::PI};
pub trait Math {
    fn math(&self) -> f64;
    fn print(&self) {
       println!("result: {:?}", self.math())
   }
   
}
#[derive(Default)]
pub struct Sqare (f64);
impl Math for Sqare {
    fn math(&self) -> f64 {
        &self.0 * 4.0
    }

    
}
impl Sqare {
    pub fn new() -> Self {
        println!("Type side size of sqare");
        let x = read();
        Self(x)
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

#[derive(Default)]
pub struct Rectangle (f64, f64);
impl Math for Rectangle {
 fn math(&self) -> f64 {
        (self.0 + self.1) * 2.0
   }

  
}

impl Rectangle {
    pub fn new() -> Self {
        println!("Type side size a of rectangle");
        let x = read();
        println!("Type side size b of rectangle");
        let y = read();
        Self(x, y)
    }

}

#[derive(Default)]
pub struct Circle(f64);
impl Math for Circle {
    fn math(&self) -> f64{
        (self.0 * self.0) * PI
    }

    
}

impl Circle {
    pub fn new() -> Self {
        println!("Type radius of Circle");
        let x = read();
        Self(x)
    }
    
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_sqare() {
        let x = Sqare (1.0);
        assert_eq!(x.math(), 4.0);
    }
    
    #[test]
    fn math_rectangle() ->Result<(), String>  {
        let x = Rectangle (1.0, 1.0);

        match x.math() {
            result if result == 4.0 => Ok(()),
            _ => Err("Must be 4.0".to_string())
        }

   
    }

}

