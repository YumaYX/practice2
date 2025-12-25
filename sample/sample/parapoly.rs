
fn print_len<T: AsRef<str>>(s: T) {
    println!("{}", s.as_ref().len());
}

fn main () {
    print_len("hello");
    print_len(String::from("world"));
}