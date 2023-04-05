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

    // floating point
    let f = 2.0;
    println!(" f {f}");

    let f: f32 = 3.0;
    println!(" f {f}");

    // boolean
    let t = true;
    println!(" t {t}");

    let t: bool = false;
    println!(" t {t}");

    // chars
    let c = 'z';
    println!(" c {c}");
    
    let c = 'Z';
    println!(" c {c}");

    // tuple fixed width but different types
    let tup: (u32, f64, u8, char) = (500, 2.0, 1, 'k');
    let (x,y,z,a) = tup;
    println!(" The value of a is {a}");

    let o = tup.0;
    println!(" The value of tup is {o}");

    // array : fixed width, same type
    let arr = [1,2,3,4,5];
    let arr1: [i32; 6]= [0,0,0,0,0,0];
    
    // same as  = [3,3,3,3,3]
    let arr2 = [3; 5];
    let arr2_1 = arr2[1];
    println!(" The value of arr2 is {arr2_1}");

    // get index out of bounds error on execution
    //let arr2_1 = arr2[7];

    // function call
    another_function();

    // function with 1 parameter
    another_function2(5);

    // function with 2 parameter
    another_function3(5,'k');

    // statement and expression
    another_function4(5);

    // function return value
    let f = another_function5(5);
    println!("function return value {f}");

    /*
    Multiple lines comments
    can be written here
     */

    let num = 3;

    // if
    if num < 4 {
        println!("condition true");
    }

    //if else
    if num <5{
        println!("condition true");
    } else {
        println!("condition false");
    }

    //if else if
    if num <2{
        println!("less than 2");
    } else if num < 3  {
        println!("less than 3");
    } else if num < 4  {
        println!("less than 4");
    } else{
        println!("greater than or = 4");
    }

    // if as expression
    let condition = true;

    let numb = if condition {100} else {200};
    println!("value of expression {numb}");

    //they need to be same expression
    // below have 2 diff types
    //if condition {100} else {"200"};

    //loop and break
    let mut van = 0;
    loop {
        van += 1;
        if van == 200000 {
            println!("Vanshi is cool {van}");
            break;
        }
    }

    println!("break return value");
    let mut van1 = 0;
    let van2 = loop {
        van1 += 1;
        if van1 >= 200000 {
            break van1+1;
        }
    };
    println!("loop expression value {van2}");

    //multi loop label
    let mut count = 0;
    'count_outer: loop {
        println!("count = {count}");
        let mut remainng = 10;
        loop{
            println!("remainng {remainng}");
            if remainng == 9 {
                break;
            }
            if count == 2 {
                break 'count_outer;
            }
            remainng -=1;
        }
        count +=1;
    }
    println!("end of count {count}");

    // while loop
    let mut numb1 = 3;

    while numb1 !=0 {
        println!("{numb1}");
        numb1 -= 1;
    }

    // for loop
    let a = [10,20,22,30,40,55,60];
    for element in a{
        println!("values via for {element}");
    }

    // for range
    for elem in (1..4).rev() {
        println!("range {elem}");
    }

    // memory safety
    let s1 = String::from("Kalidass");
    let s2=s1;

    // this can not be done as now s1 is no longer aailable
    //println!("{s1}");

    // clone copy
    let s3 = String::from("Kalidass Test");
    let s4 = s3.clone();

    println!("s3 {s3}, s4 {s4}");

    //trasnfer ownership
    let s = String::from("Kalidass transfer ownership");
    take_ownership(s);
    
    //s is no longer available as it has been moved
    //println!("{s}");

    // return ownership
    let s2 = String::from("Kalidass return ownership");
    let s3 = give_ownership(s2);
    println!("Ownership returned to {s3}");

    let x = 5;

    make_copy(x);

    println!("Original value still present as {x} in stack");

    //return multiple values ownership
    let (s4, len) = calculate_length(s3);
    println!("The length of {} is {}", s4, len);

    // passing reference rather than move

    let s5 = String::from("Pass reference example");

    let len1 = calculate_length1(&s5);

    //s5 not moved as ref passed
    println!("the length of '{}' is {} as only ref passed",s5,len1);

    let mut s6 = String::from("Pass mutable reference example");

    // there can be only 1 mutable ref 
    change_refernce(&mut s6);
    println!("The String '{}' is passed as mutable ref",s6);

    //immutable reference, mutiple ref allowed
    let s6 = String::from("immutable reference, mutiple ref allowed");
    let r1 = &s6;
    let r2 = &s6;

    //mutable ref not allowed
    //let r3 = &mut s6;

    println!("{} | {} | {}", s6,r1,r2);

    //mutable ref after no more ref allowed
    let mut s7 = String::from("mutable reference, mutiple ref allowed");
    let r4 = &s7;
    let r5 = &s7;
    println!("{} | {} | {}", s7,r4,r5);

    let r6 = &mut s7;
    println!("{}",r6);
    //not allowed
    //println!("{} | {}",s7,r6);

    // using string slice
    let str1 = String::from("hello world");
    let hello = &str1[0..5];
    let world = &str1[6..11];
    //[..2] [6..] [..]uses default value
    println!("{},{}, {}, {}",hello, world, &str1[..2], &str1[6..]);

}

//function call
fn another_function(){
    println!("In another function now")
}
//function with 1 parameter
fn another_function2(x: i32) {
    println!("get the value {x}")
}
//function with 2 parameters
fn another_function3(x: i32, y: char) {
    println!("get the value {x} , {y}")
}

fn another_function4(x: i32) {
    // statement does not return a value so can not be used for assigning value
    //let y = (let y=6);

    // statement do not return any value
    // expression return value
    let y = {
        let m = x;
        m+1
    };

    println!("value is {y}")
}

// function return value
fn another_function5(x: i32) -> i32 {
    x + 5
    // ending in ; will cause failure as that is statement and not expression
    //x + 5;
}

// get ownership
fn take_ownership(some_string: String){
    println!("{some_string}");
}

// copy created
fn make_copy(x: i32){
    println!("Created a copy of {x}");
}

// return ownership
fn give_ownership(s2: String) -> String {
    s2
}

// passing tuple response
fn calculate_length(s3: String) -> (String, usize){
    let len = s3.len();
    (s3, len)
}

// reference
fn calculate_length1(s: &String) -> usize{
    //can not modify the string as it is only borrowed
    //s.push_str(", hello");

    s.len()
}

// mutable ref
fn change_refernce(s: &mut String){
    s.push_str(", test");
}