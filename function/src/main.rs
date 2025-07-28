fn main() {
    println!("Hello, world!");

    greeting();
    number(4);
    number_char(4,'h');
}

fn greeting(){
    println!("Good morning!!!");
}

fn number(x: u32){
    println!("The number is {x}");
}

fn number_char(y: i32, character : char){
    println!("The number is {y} and the character is {character}");
}