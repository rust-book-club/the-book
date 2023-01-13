fn main() {

    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2; // return a value when breaking
    //     }
    // };
    //
    // println!("The result is {result}");

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
                break 'counting_up; // break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
