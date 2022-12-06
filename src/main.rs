use std::io::Read;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let file = &arguments[1];
    let mut tfile: std::fs::File = std::fs::File::open(file).expect("Error: file not find");
    let mut srting = String::new();
    tfile.read_to_string(&mut srting).expect("Error");
    let yamlfile: serde_yaml::Value = serde_yaml::Value::String(serde_json::to_string_pretty(&srting).unwrap());
    let s: String = serde_yaml::from_value(yamlfile).unwrap();
    println!("{}", s);
}