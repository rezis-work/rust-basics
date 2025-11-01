fn main() {
    let point: (i64, i64, i64) = (0,0,0); // immutable tuple
    let mut point_mut: (i64, i64, i64) = (0,0,0); // mutable tuple
    let _unit: () = (); // unit tuple

    point_mut.0 = 1;
    point_mut.1 = 2;
    point_mut.2 = 3;

    let x = point.0;
    let y = point.1;
    let z = point.2;

    let (k, l, m) = point;

    let (n, o, p) = point_mut;


    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("k: {}, l: {}, m: {}", k, l, m);
    println!("n: {}, o: {}, p: {}", n, o, p);
}

// we cant chenge size of a tuple

