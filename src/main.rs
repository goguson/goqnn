use rand::Rng;

fn main() {
    println!("Hello, world!");

    // first element is input, second is expected output
    let training_set = [[0.0, 0.0], [1.0, 2.0], [2.0, 4.0], [3.0, 6.0], [4.0, 8.0]];

    let w = rand::thread_rng().gen_range(0..10);
    let eps = 1e-3;
    println!("\n Result: {result}")
}

fn cost(w: i32, training_set: [[f32; 2]; 5]) -> f32 {
    let mut result = 0.0;
    for input in training_set {
        let x = input[0];
        let y = x * w as f32;
        let distance = y - input[1];
        result += distance * distance;
        println!("output: {y} | expected {}", input[1])
    }
    result
}
