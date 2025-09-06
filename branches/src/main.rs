fn main() {
    let number = 3;

    if number < 5 {
        println!("cond was true")
    } else {
        println!("cond was false")
    }

    let condition = true;
    let number = if condition { 5 } else { 3 };
    println!("value of number is {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {result}");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("liftoff");

    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("val is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("liftoff!!");

    let res = fib(15);
    println!("fib 5 = {res}")
}

fn fib(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 0;
    let mut count = 0;

    while count < n {
        let tmp = a + b;
        b = a;
        a = tmp;
        count += 1;
    }
    return b;
}
