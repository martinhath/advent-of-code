extern crate serde_json;
use self::serde_json::Value;

fn get_sum(json: &Value) -> i64 {
    if json.is_object() {
        let mut sum = 0i64;
        for (_, v) in json.as_object().unwrap() {
            if v.is_string() {
                if v.as_string().unwrap() == "red" {
                return 0;
                }
            }
            sum += get_sum(v);
        }
        sum
    } else if json.is_array() {
        json.as_array().unwrap().iter().fold(0, |a, e| a + get_sum(e))
    } else if json.is_number() {
        json.as_i64().unwrap()
    } else {
        0
    }
}

pub fn day_12(input: String) {
    let data: Value = serde_json::from_str(&input).unwrap();
    println!("{}", get_sum(&data));
}
