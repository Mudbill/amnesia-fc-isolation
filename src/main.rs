fn main() {
    let _s1 = String::from("hello");

    let s2 = gives_ownership();

    let _s2 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let string = String::from("hello");
    string
}

fn takes_and_gives_back(string: String) -> String {
    string
}
