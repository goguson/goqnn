use rand::Rng;

fn main() {
    // first element is input, second is expected output
    let training_set = [[0.0, 0.0], [1.0, 2.0], [2.0, 4.0], [3.0, 6.0], [4.0, 8.0]];

    let mut weight = rand::thread_rng().gen_range(0..10) as f64;
    println!("initial weight: {weight}");
    let eps = 1e-3;
    let learning_rate = 1e-3;
    println!("eps: {eps}");
    println!("==================git");
    for _ in 0..1000 {
        let cost_with_epsilon = cost(weight + eps, training_set);
        let cost = cost(weight, training_set);
        let dcost = (cost_with_epsilon - cost) / eps;
        println!("cost: {cost}");
        println!("cost_eps: {cost_with_epsilon}");
        println!("dcost: {dcost}");
        weight -= learning_rate * dcost;
        println!("w: {weight}");
        println!("---");
    }
}

fn cost(weight: f64, training_set: [[f64; 2]; 5]) -> f64 {
    let mut result = 0.0;
    training_set.map(|sample| {
        let input_value = sample[0];
        let expected_value = sample[1];
        let outcome = input_value * weight as f64;
        let distance = outcome - expected_value;
        result += distance * distance;
    });
    result / training_set.len() as f64
}
