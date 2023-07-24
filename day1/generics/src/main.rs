//fn print_it<T: ToString>(x: T) {
//    println!("{}", x.to_string());
//}

use std::fmt::Debug;

fn print_it<STRING>(x: STRING)
where
    STRING: ToString + Debug,
{
    println!("{}", x.to_string());
}

fn print_two<T, U>(x: T, y: U)
where T:ToString {

}

fn main() {
    print_it("Hello");
    print_it(4);
}
