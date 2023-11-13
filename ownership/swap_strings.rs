fn main () {
    let mut s1 = String::from("Soy el mensaje 1");
    let mut s2 = String::from("Soy el mensaje 2");

    println!("s1: {},\n s2: {}", s1, s2);
    swap_strings(&mut s1, &mut s2);
    println!("s1: {},\n s2: {}", s1, s2);
}

fn swap_strings(s1: &mut String, s2: &mut String) {
    let mut aux = String::new();

    aux.push_str(s1);
    s1.clear();

    s1.push_str(s2);
    s2.clear();
    s2.push_str(&aux);
}