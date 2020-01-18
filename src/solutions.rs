// https://leetcode.com/problems/defanging-an-ip-address/

pub fn defang_ip_address(address: String) -> String {
    address.replace(".", "[.]")
}

/*

Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2
are also in arr1.

Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2.
Elements that don't appear in arr2 should be placed at the end of arr1 in ascending order.



Example 1:
Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
Output: [2,2,2,1,4,3,3,9,6,7,19]

*/
pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut original_vec1 = arr1.to_vec();
    let mut output: Vec<i32> = vec![];
    for i in 0..arr2.len() {
        let num: i32 = arr2[i];
        for j in 0..arr1.len() {
            if arr1[j] == num {
                output.push(arr1[j]);
                original_vec1 = original_vec1.into_iter().filter(|&elem| elem != arr1[j]).collect();
            }
        }
    }
    original_vec1.sort();
    output.append(&mut original_vec1);
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_defang_ip_address() {
        assert_eq!(defang_ip_address("255.100.50.0".to_string()), "255[.]100[.]50[.]0");
    }

    #[test]
    fn test_relative_sort_array() {
        assert_eq!(relative_sort_array(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6]), vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19])
    }
}