// Converting to a String
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let str_x = x.to_string();
    let len_x = str_x.len();

    if len_x == 1 {
        return true;
    }

    let mut index = 0;

    for ch in str_x.chars() {
        match ch.eq(&str_x.chars().nth(len_x - 1 - index).unwrap()) {
            false => return false,
            _ => ()
        }
        index += 1;
    }

    return true;
}

fn is_palindrome_no_str(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut reversed = 0;
    let mut temp = x;

    while temp > 0 {
        let digit = temp % 10;
        reversed = reversed * 10 + digit;
        temp /= 10;
    }

    reversed == x
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome() {
        let result = is_palindrome(323);
        assert!(result);

        let result = is_palindrome_no_str(323);
        assert!(result);

        let result = is_palindrome_no_str(1);
        assert!(result);
    }

    #[test]
    fn not_palindrome() {
        let result = is_palindrome(123);
        assert_eq!(result, false);

        let result = is_palindrome_no_str(123);
        assert_eq!(result, false);
    }
}
