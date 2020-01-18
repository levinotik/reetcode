// https://leetcode.com/problems/defanging-an-ip-address/

pub fn defang_ip_address(address: String) -> String {
    address.replace(".", "[.]")
}

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    arr1
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
        assert_eq!(relative_sort_array(vec![2,3,1,3,2,4,6,7,9,2,19], vec![2,1,4,3,9,6]), vec![2,2,2,1,4,3,3,9,6,7,19])
    }

}