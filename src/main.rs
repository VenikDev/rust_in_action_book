use std::ops::Index;

fn greet_world() {
    println!("Hello world");
    let southern_getmany = "Eine Begrüßung";
    let japan = "南ドイツ";
    let regions = [southern_getmany, japan];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    let penguin_data = "\
    Common Name, 33
    Littile penguin, 65
    Invalide data
    ";

    let recorgs = penguin_data.lines();

    for (idx, item) in recorgs.enumerate() {
        if idx == 0 || item.trim().is_empty() {
            continue;
        }

        let fields: Vec<_> = item
            .split(',')
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertion) {
            eprintln!("debug: {:?} -> {:?}", item, fields)
        }

        let name = fields[0];

        if let Ok(lenght) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, lenght)
        }
    }
}