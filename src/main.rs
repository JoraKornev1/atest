use atest::*;
fn main() {
    let square = Sqare::new();
    let reactangle = Rectangle::new();
    let circle = Circle::new();
    let mut vecs: Vec<&dyn Math> = Vec::new();
    vecs.push(&square);
    vecs.push(&reactangle);
    vecs.push(&circle);


    for i in vecs {
        i.print();
    }

   


       
}