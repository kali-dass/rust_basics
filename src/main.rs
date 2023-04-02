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

    //fixed width but different types
    let tup: (u32, f64, u8, char) = (500, 2.0, 1, 'k');
    let (x,y,z,a) = tup;
    println!(" The value of a is {a}");

    let o = tup.0;
    println!(" The value of tup is {o}");

    // fixed width, same type
    let arr = [1,2,3,4,5];
    let arr1: [i32; 6]= [0,0,0,0,0,0];
    
    // same as  = [3,3,3,3,3]
    let arr2 = [3; 5];
    let arr2_1 = arr2[1];
    println!(" The value of arr2 is {arr2_1}");

    // get index out of bounds error on execution
    //let arr2_1 = arr2[7];

    another_function();

    another_function2(5);

    another_function3(5,'k');

    another_function4(5);

}

fn another_function(){
    println!("In another function now")
}
fn another_function2(x: i32) {
    println!("get the value {x}")
}
fn another_function3(x: i32, y: char) {
    println!("get the value {x} , {y}")
}
fn another_function4(x: i32) {
    // statement does not return a value so can not be used for assigning value
    //let y = (let y=6);

    let y = {
        let m = x;
        m+1
    };

    println!("value is {y}")
}