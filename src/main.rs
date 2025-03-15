fn main() {
    let tup = (400,  3.1, 3);
    let (_x, y, _z) = tup;
    println!("the value of y is :{y}" ) ;

    let x = (500, 5.6, 4);
    let a = x.2;
    println!("the value of a is :{a}" ) ;
    let b = x.1;
    println!("the value of b is :{b}" ) ;
}
