fn main() {
    let mut years: [i32; 3] = [2020, 2021, 2022];

    let first_year = years[0];
    let second_year = years[1];
    let third_year = years[2];

    println!("first_year: {}, second_year: {}, third_year: {}", first_year, second_year, third_year);

    years[0] = 2023;
    println!("first_year: {}", years[0]);

    for year in years.iter() {
        println!("year: {}", year);
    }
}


