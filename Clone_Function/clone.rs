fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str("Ahmad Faraz");
    return cloned_string;
}

fn main() {
    let orignal_string = String::from("Orignal Hello World!");
    let modified_string = modify_string(&orignal_string); // created a deep copy

    println!("Orignal String : {}", orignal_string);
    println!("Modified String : {}", modified_string);

}

//Orignal String : Orignal Hello World!
// Modified String : Orignal Hello World!Ahmad Faraz
