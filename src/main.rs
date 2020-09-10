fn main() {
    let mut x=5;
    println!("the value of x is: {}",x);
    x=6;
    println!("the value of x is: {}",x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    const MAX_POINTS: u32 = 100_000; //constant must be capital
    println!("the value of the constant is {}", MAX_POINTS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length is: {}",spaces);
    //integer data types(signed i + unsigned u)
    let n: u32 = 24;

    //float data type
    let a = 2.0; // type:f64
    let b: f32 = 3.0; // f32

    let t = true;
    let f: bool = false; // with explicit type annotation

    //character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = tup.1; //access a tup member by index

    let tup = (500, 6.4, 1);
    let (xx, yy, zz) = tup;
    println!("The value of y is: {}", yy);

    //the array type
    let arr = [1, 2, 3, 4, 5];
    let a = [3; 5]; //5 elements that will all be set to the value 3 initially.
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //i32 is the type of each element; the number 5 number of elements.
    let first = a[0]; 

    //functions with args
    another_function(5,6);

    //function with return value
    let x = five();
    println!("The value of x is: {}", x);

    let x=plus_one(5);
    println!("the value of x is: {}",x);

    //control flow (if,else)
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {5} else {6};  //args of if and else should be the same type ex: both int
    println!("the value of the number is {}", number);

    //infinite loop use: loop {}
    let mut counter = 0;
    let result = loop {
        counter+=1;
        if counter == 10 { break counter *2; }
    };
    println!("the result is {}", result);
    //while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for num in (1..4).rev() {
        println!("{}!",num);
    }
    println!("outside");

    //string type (allocated on the heap not stack, useful for user input)
    let mut s = String::from("hello");
    s.push_str(", world");// push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    //The double colon (::) is an operator that allows us to namespace this particular "from" function 
    //under the "String" type rather than using some sort of name like "string_from".
    let t1 = String::from("hi");
    let t2 = t1.clone();
    print!("t1 is {} and t2 is {}",t1,t2);

}
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
} //In function signatures, you must declare the type of each parameter.

fn five() -> i32 {
    5
}
fn plus_one(g: i32) -> i32 {
    g+1 //no semicolun cuz then it will be a statement instead of an exprission
}


