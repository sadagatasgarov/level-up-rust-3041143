
fn main() {
    let list = vec![1.0, 5.0, 6.0,7.0];
    println!("{:.1}", median_calc(list).unwrap())
}

fn median_calc(mut vec: Vec<f32>) -> Option<f32> {
    if vec.is_empty() {
        return None;
    }

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n_elements = vec.len();
    let middle: usize = n_elements / 2;
    let vec_is_even = n_elements % 2 == 0;
    let med  =  if vec_is_even {
        // mean  / average of the middle-two elements
        Some((vec[middle] + vec[middle - 1]) / 2.0)
    } else {
        Some(vec[middle])
    };

    med
}