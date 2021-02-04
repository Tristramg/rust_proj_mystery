fn main() {
    let from = "EPSG:28992";
    let to = "EPSG:4326";
    let amersfoort_to_wgs84 = proj::Proj::new_known_crs(&from, &to, None).unwrap();
    let result = amersfoort_to_wgs84
        .convert(geo_types::Point::new(228377.0, 527847.0))
        .unwrap();
    println!("computed: {} {}", result.x(), result.y());
    println!("local   : {} {}", 6.473556233092426, 52.733020858848604);
    println!("tests   : {} {}", 6.473572155087817, 52.733093297726396);
}
