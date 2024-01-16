
// fn xo(string: &'static str) -> bool {
//     let (mut o_count, mut x_count) = (0, 0);
//     let string = string.to_lowercase();

//     for i in string.chars() {
//         match i {
//             'x' => x_count += 1,
//             'o' => o_count += 1,
//             _ => continue,
//         }
//     }

//     if x_count == o_count {
//         return true;
//     } else {
//         return false;
//     }
//   }


// better function-----------------------------------


fn xo(string: &'static str) -> bool {
    let string = string.to_lowercase();

    let o_count = string.chars().filter(|&c| c == 'o' ).count();
    let x_count = string.chars().filter(|&c| c == 'x' ).count();
    
    return o_count == x_count
}


// community better fn -------------
// fn xo(string: &'static str) -> bool {
//     string.chars().fold(0, |a, c|{
//       match c {
//         'x' | 'X' => a + 1,
//         'o' | 'O' => a - 1,
//         _ => a
//       }
//     }) == 0
//   }


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
