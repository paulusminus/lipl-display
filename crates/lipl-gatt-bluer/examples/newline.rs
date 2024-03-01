fn main() {
    let s = "Hallo\nDit is een test";
    let output = serde_json::to_string(&s).unwrap();
    println!("{}", &output);
}
