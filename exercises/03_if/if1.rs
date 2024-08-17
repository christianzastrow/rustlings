use core::num;

fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if (a > b) {
        return a;
    }
    else if (b > a) {
        return b;
    }
    else {
        return a;
    }
}

fn main() {
    // You can optionally experiment here.
    let num1: i32 = 2;
    let num2: i32 = 20;
    println!("The greater number is {}", bigger(num1, num2));
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
