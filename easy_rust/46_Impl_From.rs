use std::fmt::Display;

fn print_vec<T: Display>(input : &Vec<T>){
    for item in input {
        print!("{item} ")
    }
    println!();
}
fn main() {
    let array_vec =  Vec::from([8, 9, 10]);
    print_vec(&array_vec);

    let str_vec = Vec::from("What kind of vec is this?");
    print_vec(&str_vec);

    let a = vec![b'W', b'h', b'a', b't'];
    print_vec(&a);

    let string_vec = Vec::from("What kind of Vec is a String Vec?");
    print_vec(&string_vec);
}