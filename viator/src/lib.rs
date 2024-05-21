pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn main()
{
    let mut left = 14;
    let mut right = 16;

    let result = add(left, right);

    let variable = left + 2;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
