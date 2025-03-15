use std::io;
fn main() {
    let tup = (400,  3.1, 3);
    let (_x, y, _z) = tup;
    println!("the value of y is :{y}" ) ;

    let x = (500, 5.6, 4);
    let a = x.2;
    println!("the value of a is :{a}" ) ;
    let b = x.1;
    println!("the value of b is :{b}" ) ;

   

    let naveed: [u32; 10] = [6; 10];
    println!("the value of naveed is :{:?}", naveed) ;

    let h = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = h[index];

    println!("The value of the element at index {index} is: {element}");
}
