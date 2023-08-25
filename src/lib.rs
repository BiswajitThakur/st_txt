use std::collections::HashMap;

pub fn set_style(input_str: String, st: &HashMap<char, String>) -> String {
    let mut r = String::new();
    for i in input_str.chars() {
        match st.get(&i) {
            Some(v) => r.push_str(v),
            None => r.push_str(&i.to_string()),
        };
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_style() {
        let s: HashMap<char, String> = HashMap::from([
            ('a', "AA".to_string()),
            ('b', "BB".to_string()),
            ('B', "bb".to_string()),
            ('6', "sx".to_string()),
        ]);
        assert_eq!(
            set_style("Biswajit 560 @ 7^#".to_string(), &s),
            "bbiswAAjit 5sx0 @ 7^#".to_string()
        );
    }
}
