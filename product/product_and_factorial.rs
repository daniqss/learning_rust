fn init_array (array: [i32;6]) -> [i32;6] {
    let mut array = [0;6];

    for i in 0..array.len() {
        array[i] = i + 1;
    }

    array
}

fn product (array: [i32;6]) -> i32{
    let mut x = 1;

    for i in 0..array.len() {
        x *= array[i];
    }

    x
}

fn factorial (n : i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let array: [i32;6] = [1, 2, 3, 4, 5, 6];
    let mut random_array :[i32;6];

    init_array(random_array);

    println!("The result is {}", product(array));
    println!("The factorial of 6 is {}", factorial(6));
}