pub fn run(list: &[i32], number: i32) -> bool {
    let mut cycles = 0; // Count cycles

    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        cycles += 1;  // Count cycles

        let pivot = (low + high) / 2;

        if number == list[pivot] {
            println!("Found input '{}' in list! It took {} cycles", number, cycles);
            return true;
        }

        if number > list[pivot] {
            low = pivot + 1;
        } else if number < list[pivot] {
            high = pivot - 1;
        }
    }

    println!("Did not find input '{}' in list! It took {} cycles", number, cycles);
    false
}
