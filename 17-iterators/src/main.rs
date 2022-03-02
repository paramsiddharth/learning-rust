fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    for val in v_iter {
        println!("Got: {}", val);
    }
    
    let plus_1: Vec<_> = v.iter().map(|x| x + 1).collect();

    for val in plus_1 {
        println!("Got (plus 1): {}", val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iteration_demonstration() {
        let v = vec![1, 2, 3];

        let mut v_iter = v.iter();

        assert_eq!(v_iter.next(), Some(&1));
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&3));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn iteration_sum() {
        let v = vec![1, 2, 3];

        let v_iter = v.iter();

        let total: i32 = v_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iteration_map() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
        assert_eq!(v2, vec![2, 3, 4]);

        let v3: Vec<_> = v2.iter()
            .filter(|x| *x % 2 == 0)
            .map(|x| *x)
            .collect();

        assert_eq!(v3, vec![2, 4]);
    }
}