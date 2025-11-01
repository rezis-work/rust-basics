struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City { description: (format!("{} *non-coastal* with residents is not a coastal city", residents)), residents, is_coastal }
    }
}

fn main() {
    let rustville: City = new_city(123, false);
    let springfield: City = new_city(75000, false);

    println!("This city can be described as: {}", rustville.description);
    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }

    println!("\nThis city can be described as: {}", springfield.description);
    if springfield.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
