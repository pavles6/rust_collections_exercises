use rust_collections_exercises::find_median_and_mode::get_median_and_mode;

fn main() {
    let mut input = vec![
        78, 45, 12, 90, 34, 67, 56, 23, 87, 1, 99, 73, 89, 43, 21, 10, 55, 79, 88, 14, 66, 32, 50,
        71, 91,
    ];

    let (median, mode) = get_median_and_mode(&mut input);

    println!("Input: {:?}\nMedian: {}\nMode: {}", input, median, mode);
}
