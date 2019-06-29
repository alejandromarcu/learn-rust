struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut loc = Vec::new();

        for trip in trips {
            let n = trip[0];
            let st = trip[1];
            let fi = trip[2];

            loc.push((st, n));
            loc.push((fi, -n));
        }


        loc.sort();
        let mut current = 0;
        for (_, n) in loc {
            current += n;
            if current > capacity {
                return false;
            }
        }

        true 
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let trips = vec![vec![2,1,5],vec![3,3,7]];
        assert_eq!(false, Solution::car_pooling(trips, 4));
    }

    #[test]
    fn test2() {
        let trips = vec![vec![2,1,5],vec![3,3,7]];
        assert_eq!(true, Solution::car_pooling(trips, 5));
    }
}


fn main() {
        let trips = vec![vec![2,1,5],vec![3,5,7]];
        assert_eq!(false, Solution::car_pooling(trips, 4));
        
}