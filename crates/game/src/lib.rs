pub fn initialize() {
    let content = omniciv_data::build().unwrap();
    println!("{content:?}");
}
