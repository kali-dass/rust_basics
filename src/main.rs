fn main() {
    
    // imutables
    let x = 5;
    println!("value of x is {x}");
    // can't assign values to immutable variable
    //x = 6;
    //println!("value of x is {x}");

    // mutable 
    let mut y = 5;
    println!("value of y is {y}");
    // assign values to mutable variable
    y = 6;
    println!("value of y is {y}");

    //shadow
    let x = x +1;
    {
        let x = x +1;
        println!("value of x in inner scope {x}");
    }
    println!("value of x in outter scope {x}");

    // specify data type
    let guess: u32 = "42".parse().expect("Not a number");
    println!(" guess {guess}");

    let f = 2.0;
    println!(" f {f}");

    let f: f32 = 3.0;
    println!(" f {f}");

    let t = true;
    println!(" t {t}");

    let t: bool = false;
    println!(" t {t}");

    let c = 'z';
    println!(" c {c}");
    
    let c = 'Z';
    println!(" c {c}");

    let tup: (u32, f64, u8, char) = (500, 2.0, 1, 'k');
    let (x,y,z,a) = tup;
    println!(" The value of a is {a}");

    let o = tup.0;
    println!(" The value of tup is {o}");

}
