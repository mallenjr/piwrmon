use regex::Regex;

pub fn get_field_reg(data: &String, re: &str) -> String {
    let re = Regex::new(re).expect("failed to compile regex");

    let cap = re.captures(&data)
        .expect("failed to parse regex field");

    return String::from(&cap[1]);
}