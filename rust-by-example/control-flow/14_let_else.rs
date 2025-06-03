use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair : `{}`", s);
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer : `{}`", count_str);
    };
    (count, item)
}

// Without let else
fn get_count_item2(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair : `{}`", s),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer : {}", count_str);
    };
    (count, item)
}
fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    assert_eq!(get_count_item2("-11 chairs"), (7, "chairs"));
}