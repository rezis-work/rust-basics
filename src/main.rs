enum CitySize {
    Town,
    City,
    Metropolis,
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> Self {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents
                )
            }
            CitySize::City => {
                let residents = 10_000;

                (
                    format!("a *city* of approximately {} residents", residents),
                    residents
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;

                (
                    format!("a *metropolis* of approximately {} residents", residents),
                    residents
                )
            }
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    let tbilisi = City::new(CitySize::Metropolis, true);

    println!("This city is called {} and has {} residents", tbilisi.description, tbilisi.residents);

    if tbilisi.residents > 100_000 {
        println!("This city is a city!");
    } else {
        println!("This city is a town!");
    }
}