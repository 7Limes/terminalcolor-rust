const COLORS: [char; 9] = ['k','r','g','y','b','m','c','w','0'];

/// Convert shorthand color codes into actual color codes.
/// Best if used in conjunction with the `format!` macro.
/// 
/// # Syntax
/// To color a substring, prefix it with one of the following:
/// - `$[char]` - Foreground color
/// - `#[char]` - Background color
///
/// `[char]` must be one of the characters found in the `COLORS` constant.
/// Here is the character for each color:
/// - `black` - `k`
/// - `red` - `r`
/// - `green` - `g`
/// - `yellow` - `y`
/// - `blue` - `b`
/// - `magenta` - `m`
/// - `cyan` - `c`
/// - `white` - `w`
/// - `reset` - `0`
///
/// You can also capitalize any of those characters to make the color bright (this may not have an effect in all terminals).
/// You can use as many of these codes as you want in a string.
///
/// Example
/// ```
/// use terminalcolor as tc;
/// let s = tc::convert(&String::from("$rHello, $Bworld!"));
/// println!("{}", s);
/// ```
pub fn convert(s: &String) -> String {
    let mut out = String::new();
    let mut i: usize = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if i >= s.len()-1 {
            out.push(c);
            break;
        }
        let color_code = s.chars().nth(i+1).unwrap();
        if (c != '$' && c != '#') || !COLORS.contains(&color_code.to_ascii_lowercase()) {
            out.push(c);
            i += 1;
            continue;
        }
        if color_code == '0' {
            out.push_str("\x1b[0m");
            i += 2;
            continue;
        }
        let color_index = COLORS.iter().position(|&x| x == color_code.to_ascii_lowercase()).unwrap();
        let mut add = match c {
            '$' => 30,
            '#' => 40,
            _ => 30
        };
        add += match color_code.is_uppercase() {
            true => 60,
            false => 0
        };
        let code = &format!("\x1b[{}m", add+color_index)[..];
        out.push_str(code);
        i += 2;
    }
    out
}

/// Print string with color. Syntax specified in `convert()` documentation.
pub fn printc(s: &String) {
    print!("{}", convert(s));
}

/// Print string with color with a newline. Syntax specified in `convert()` documentation.
pub fn printlnc(s: &String) {
    println!("{}", convert(s));
}
