fn main() {
    pub fn is_valid(mut s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        while s.contains("()") || s.contains("{}") || s.contains("[]") {
            s = s.replace("()", "");
            s = s.replace("[]", "");
            s = s.replace("{}", "");
        }

        return s.len() == 0;
    }

    let result = is_valid("[](){}".to_owned());

    println!("{:?}", result);
}
