use std::io;

fn main() {
    println!("Enter your weigth (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weigth: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weigth_on_mars(weigth);

    println!("Weight on Mars: {}kg", mars_weight);
}


fn calculate_weigth_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
