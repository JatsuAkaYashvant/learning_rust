fn main() {
    let mut x =5;
    println!("the value of x is : {x}");
    x = 6;
    println!("the value of x is : {x}");

    const THREE_HOURS_IN_SECOND : u32 = 3 * 60 * 60;
    println!("The constant is {THREE_HOURS_IN_SECOND}");

    let y = 6;

    let y = y + 1 ;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of x is: {x}");
}
