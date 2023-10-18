fn main() {
    // Tuple type
    let x: (i32, f64, u8) = (500, 6.4, 1);
    // Assing to a new variable the 0 index of the tuple x
    let five_hundred = x.0;
    // Assing to a new variable the 1 index of the tuple x
    let six_point_four = x.1;
    // Assing to a new variable the 2 index of the tuple x
    let one = x.2;
    println!("the first item in x tuple is: {five_hundred}\nthe second is: {six_point_four}\nand the third is: {one}");
}
