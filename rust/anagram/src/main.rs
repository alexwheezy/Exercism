fn main() {
    let v = vec!["ΒΓΑ", "γβα"];
    for s in v.iter() {
        println!("{:?}", s.escape_unicode().to_string());
    }
}
