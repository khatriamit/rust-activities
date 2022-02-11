// implement bubble sort algorithm in RUST

/*
    Bubble Sort is the simplest sorting algorithm that
    works by repeatedly swapping the adjacent elements
    if they are in wrong order.
*/

fn main() {
    let mut values = [1, 4, 8, 2, 0, 8, 12];

    for i in 0..values.len() {
        for j in ((i + 1)..values.len()).rev() {
            if values[j - 1] > values[j] {
                swap_values(&mut values, j - 1, j);
            }
            println!("{:?}", values);
        }
    }
}

fn swap_values(my_list: &mut [i32; 7], i: usize, j: usize) {
    let temp: i32;
    temp = my_list[i];
    my_list[i] = my_list[j];
    my_list[j] = temp;
}
