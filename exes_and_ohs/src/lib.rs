
fn xo(string: &'static str) -> bool {
    let (mut o_count, mut x_count) = (0, 0);
    let string = string.to_lowercase();

    for i in string.chars() {
        match i {
            'x' => x_count += 1,
            'o' => o_count += 1,
            _ => continue,
        }
    }

    if x_count == o_count {
        return true;
    } else {
        return false;
    }
  }

#[cfg(test)]

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
  }
