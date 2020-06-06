fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫

    let tup = (500, 6.4, 1);

    let (x, y, mut z) = tup;

    println!("The value of z is: {}", z);

    z = 10;

    println!("The value of z is: {}", z);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);   // 要素の値は{}です
}
