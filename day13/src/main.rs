use serde_json::Value;
use std::cmp::Ordering;

fn cmp(v1: &Value, v2: &Value) -> Ordering {
    match (v1, v2) {
        (Value::Number(v1), Value::Number(v2)) => v1.as_i64().unwrap().cmp(&v2.as_i64().unwrap()),
        (Value::Array(v1s), Value::Array(v2s)) => {
            let (len1, len2) = (v1s.len(), v2s.len());
            for (v1, v2) in v1s.into_iter().zip(v2s) {
                let c = cmp(v1, v2);
                if !c.is_eq() {
                    return c;
                }
            }
            len1.cmp(&len2)
        }
        (Value::Number(v1), v2s @ Value::Array(_)) => {
            cmp(&Value::Array(vec![Value::Number(v1.clone())]), v2s)
        }
        (v1s @ Value::Array(_), Value::Number(v2)) => {
            cmp(v1s, &Value::Array(vec![Value::Number(v2.clone())]))
        }
        _ => unreachable!(),
    }
}
fn main() {
    let mut ans = 0;
    let mut lines = std::io::stdin().lines();
    let mut i = 0;
    let mut vals = vec![];
    loop {
        i += 1;
        let line1 = lines.next().unwrap().unwrap();
        let line2 = lines.next().unwrap().unwrap();

        let v1: Value = serde_json::from_str(&line1).unwrap();
        let v2: Value = serde_json::from_str(&line2).unwrap();

        if cmp(&v1, &v2) == Ordering::Less {
            ans += i;
        }

        vals.push(v1);
        vals.push(v2);

        if lines.next().is_none() {
            break;
        }
    }
    let p1: Value = serde_json::from_str("[[2]]").unwrap();
    let p2: Value = serde_json::from_str("[[6]]").unwrap();
    vals.push(p1.clone());
    vals.push(p2.clone());
    vals.sort_by(|v1, v2| cmp(v1, v2));
    println!("{vals:?}");
    let idx1 = vals.iter().position(|x| x == &p1).unwrap() + 1;
    let idx2 = vals.iter().position(|x| x == &p2).unwrap() + 1;
    println!("part1: {ans}");
    println!("part2: {}", idx1 * idx2);
}
