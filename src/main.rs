fn main() {
    let should_we_go_fast = true;
    let should_we_go_slow = false;

    let comparison = 5 == 3;

    let cat_name = "tommy";
    let cats_age = 10;

    let cat_is_old = if cats_age > 10 { true as u8 } else { false as u8 };

    if cat_name == "tommy" {
        println!("tommy is a good boy");
    } else {
        println!("Hello stranger cat");
    }

    println!("should_we_go_fast is {}", should_we_go_fast as u8);
    println!("should_we_go_slow is {}", should_we_go_slow as u8);
    println!("comparison is {}", comparison as u8);
    println!("cat_is_old is {}", cat_is_old);
}

