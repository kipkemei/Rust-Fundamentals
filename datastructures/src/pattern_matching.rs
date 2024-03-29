pub fn pattern_matching() {
    for x in 0..18 {
        println!("{}: I have {} dreams", x, how_many(x));
    }
    let point = (6,9);
    match point {
        (0,0) => println!("Origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        (_,y) => println!("(?, {}", y),
        //(ref mut x,y) => println!("({}, {}", x, y) 
    }

    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor{black:255,..}=> println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _=> ()
}
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        3...10 => "a few",
        12 => "a dozen",
        11...15 => "many",
        _ if (x % 2 == 0) => "some",
        _ => "big"
    }
}

enum Color {
    Red, 
    Green, 
    Blue,
    RgbColor(u8, u8, u8), //tuple
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}, //Struct
}
