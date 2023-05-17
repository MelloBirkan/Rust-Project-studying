fn main() {
    // Haversine formula
    // Calcular distancia de 2 aeroportos
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0; // Const não pode ser shadowed

    // Cleveland Hopkins International Airport
    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.81111;

    // Salt Lake City International Airport
    let kslc_latitude_degrees: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    // Convert degrees to radians
    //f64 .to_radians()
    let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
    let kslc_latitude_radians = kslc_latitude_degrees.to_radians();

    let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
    let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
        + kcle_latitude_radians.cos()
        * kslc_latitude_radians.cos()
        * f64::powi((delta_longitude / 2.0).sin(), 2);
    let central_angle = 2.0 * inner_central_angle.sqrt().asin();

    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    println!("Distance between airports is {:.1} km", distance); // {:.1} = 1 casa decimal
}
