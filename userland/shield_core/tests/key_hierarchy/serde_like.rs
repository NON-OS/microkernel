//! A reader for the oracle's flat JSON of integers and integer arrays.

use std::collections::BTreeMap;
pub enum Value {
    Num(u64),
    Arr(Vec<u64>),
}
impl Value {
    pub fn as_u64(&self) -> u64 {
        match self {
            Value::Num(n) => *n,
            Value::Arr(_) => panic!("expected a number"),
        }
    }
    pub fn as_array(&self) -> &Vec<u64> {
        match self {
            Value::Arr(a) => a,
            Value::Num(_) => panic!("expected an array"),
        }
    }
}
pub fn cases(json: &str) -> Vec<BTreeMap<String, Value>> {
    let body = &json[json.find("\"cases\"").expect("cases")..];
    let mut out = Vec::new();
    for obj in body.split('{').skip(1) {
        let obj = &obj[..obj.find('}').expect("close")];
        let mut m = BTreeMap::new();
        for field in split_fields(obj) {
            let (k, v) = field.split_once(':').expect("pair");
            let k = k.trim().trim_matches('"').to_string();
            let v = v.trim();
            let value = if v.starts_with('[') {
                Value::Arr(
                    v.trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .map(|n| n.trim().parse().expect("u64"))
                        .collect(),
                )
            } else {
                Value::Num(v.parse().expect("u64"))
            };
            m.insert(k, value);
        }
        out.push(m);
    }
    out
}
fn split_fields(obj: &str) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0, 0);
    for (i, ch) in obj.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(&obj[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&obj[start..]);
    out.into_iter().filter(|f| !f.trim().is_empty()).collect()
}
