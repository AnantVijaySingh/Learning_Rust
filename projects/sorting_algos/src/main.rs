fn main() {
    println!("Sorting an array of numbers using different sorting algorithms from MIT 6.006");

    let array_to_be_sorted = [73, 12, 100, 23, 04, 88, 21, 100, 11, 15];
    // let array_to_be_sorted = [5, 5, 1, 4, 3];

    println!("Sorted version of {:?} is {:?}", array_to_be_sorted, selection_sort_1(array_to_be_sorted));
}


fn selection_sort_1(mut array: [i32; 10]) -> [i32; 10] {

    println!("Sorting {:?}", array);

    let mut length = array.len();

    for _index in 0..array.len() {

        // println!("Finding the largest value in {:?}", &array[0..length-1]);

        let mut index_of_max_value = 0;

        for i in 1..length-1 {

            // println!("Is {} less than {}", array[index_of_max_value], array[i]);
            if array[index_of_max_value] < array[i] {
                index_of_max_value = i;
            }
        }

        if array[index_of_max_value] > array[length-1] {
            // println!("{} is greater than {}, so we'll swap", array[index_of_max_value], array[length-1]);
            let temp = array[length-1];
            array[length-1] = array[index_of_max_value];
            array[index_of_max_value] = temp;
        }

        println!("Semi sorted {:?}", array);

        if length > 1 {
            length -= 1;
        }
    }

    array
}