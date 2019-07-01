struct Solution;


// Not very happy with having to check for dest==0 or src==0, but otherwise it overflows.
// I could change dest and src to be i32, but then I need to cast them to usize many times
// or add an extra variable for each.
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
       let N = arr.len();
       let Z = arr.iter().filter(|&n| *n == 0).count();

       let mut src = N - 1;
       let mut dest = N - 1 + Z;

       while src != dest {
           if dest < N {
               arr[dest] = arr[src];
           }
           dest -= 1;
           
           if arr[src] == 0 {
               if dest < N {
                    arr[dest] = arr[src];
               }
               if dest == 0 {
                   break;
               }
               dest -= 1;
           }
        
           if src == 0 { 
               break
           }
           src -= 1;
       }
    }
}

fn main() {
    let mut v = vec![1,0,2,3,0,4,5,0];
    Solution::duplicate_zeros(&mut v);

    let expected = vec![1,0,0,2,3,0,0,4];
    assert_eq!(expected, v);
}
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let mut v = vec![1,0,2,3,0,4,5,0];
        Solution::duplicate_zeros(&mut v);

        let expected = vec![1,0,0,2,3,0,0,4];
        assert_eq!(expected, v);
    }

    
    #[test]
    fn test2() {
        let mut v = vec![1,2,3];
        Solution::duplicate_zeros(&mut v);

        let expected = vec![1,2,3];
        assert_eq!(expected, v);
    }

    #[test]
    fn test3() {
        let mut v = vec![0,1,7,6,0,2,0,7];
        Solution::duplicate_zeros(&mut v);

        let expected = vec![0,0,1,7,6,0,0,2];
        assert_eq!(expected, v);
    }

    #[test]
    fn test4() {
        let mut v = vec![0,0,0,0,0,0,0];
        Solution::duplicate_zeros(&mut v);

        let expected = vec![0,0,0,0,0,0,0];
        assert_eq!(expected, v);
    }

    #[test]
    fn test5() {
        let mut v = vec![8,4,5,0,0,0,0,7];
        Solution::duplicate_zeros(&mut v);

        let expected = vec![8,4,5,0,0,0,0,0];
        assert_eq!(expected, v);
    }
}