// https://leetcode.com/problems/defanging-an-ip-address/

pub fn defang_ip_address(address: String) -> String {
    address.replace(".", "[.]")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_defang_ip_address() {
        assert_eq!(defang_ip_address("255.100.50.0".to_string()), "255[.]100[.]50[.]0");
    }
}