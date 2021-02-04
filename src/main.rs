use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Point {
    lon: f64,
    lat: f64,
    category: String,
}

fn main() {
    let file = std::fs::File::open("POINTXXXXX.TMI").expect("The file is missing…");

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .trim(csv::Trim::All)
        .from_reader(file);
    let from = "EPSG:28992";
    let to = "EPSG:4326";
    let amersfoort_to_wgs84 = proj::Proj::new_known_crs(&from, &to, None).unwrap();
    for _point in rdr.deserialize::<Point>() {
        let result = amersfoort_to_wgs84
            .convert((228377.0 as f64, 527847.0 as f64))
            .unwrap();
        println!("computed: {} {}", result.x(), result.y());
    }
    println!("local   : {} {}", 6.473556233092426, 52.733020858848604);
    println!("tests   : {} {}", 6.473572155087817, 52.733093297726396);
}
