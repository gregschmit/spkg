fn profile_test() {
    let body: String = ureq::get("https://files.schmit.net/README.md")
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    println!("body = {:?}", body);
}

fn main() {
    profile_test();
}
