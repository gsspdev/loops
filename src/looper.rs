pub fn looper() {
    let mut counter = 0;

    let result = loop {
        println!("counter: {counter}");
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
        println!("result: {result}");
}

pub fn for_e_in_c() {
    let c = [1, 2, 3, 4, 5];

    for element in c {
    println!("the value is: {element}");
    }
}
