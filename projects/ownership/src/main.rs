fn main() {
    println!("Hello, world!");

    // Fixing an Unsafe Program: Returning a Reference to the Stack
    return_string();

    let mut a_string = String::from("Hello hello");
    return_a_string(&mut a_string);


    // Fixing an Unsafe Program: Not Enough Permissions
    let name = vec![String::from("Ferris")];
    println!("{}", stringify_name_with_title(&name));
    println!("{}", stringify_name_with_title_2(&name));

    // Fixing an Unsafe Program: Aliasing and Mutating a Data Structure


    // Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");

    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");

    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);

    // Fixing a Safe Program: Mutating Different Tuple Fields
    let mut name = (
        String::from("Harry"),
        String::from("Potter")
    );

    let first = &name.0;

    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    // Fixing a Safe Program: Mutating Different Array Elements
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    *x += 1;
    println!("{a:?}");

}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}




fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

fn stringify_name_with_title_2(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."

fn return_string () -> &'static str {
    "Hello Mars!"
}

fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}



