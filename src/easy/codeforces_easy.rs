/*!
## codeforces的stdin/stdout注意事项

1. 不支持dbg!宏，dbg!编译不报错但是不会显示到stdout，意味着无法通过dbg!去debug (leetcode也不支持dbg!)
2. stdin每行的分隔符是CR+LF两个byte，例如本题的测试用例8在stdin上为: [56,13,10]+EOF
3. Rust目前没有Python的os.linesep可以获取当前操作系统的换行符，不利于单元测试的跨平台

但是在mac上(terminal或IDEA)，stdin输入回车只有LF，不是codeforces的CRLF，不方便模拟codeforces上运行代码的真实环境
mac/linux: [49,48,48,13(CR),10(LF)]
codeforces: [49,48,48,10(LF)]

注意在IDEA上发送EOF的快捷键是cmd+d
*/

/// https://codeforces.com/problemset/problem/4/A
fn cf_1a_theatre_square(
    mut reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let input_split: Vec<u32> = input
        .split_whitespace()
        .map(|each| each.parse::<u32>().unwrap())
        .collect();
    let (n, m, a) = (input_split[0], input_split[1], input_split[2]);
    let num_width = u64::from(n / a) + u64::from(n % a != 0); // divmod, ceil
    let num_length = u64::from(m / a) + u64::from(m % a != 0);
    write!(writer, "{}", num_width * num_length)?;
    Ok(())
}

#[test]
fn test_cf_1a_theatre_square() {
    const TEST_CASES: [(&[u8], &[u8]); 1] = [(b"6 6 4\n", b"4")];
    for (input, expected_output) in TEST_CASES {
        let mut output = Vec::new();
        cf_1a_theatre_square(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/// https://codeforces.com/problemset/problem/4/A
/// 本题问的是一个数能否被分成两个偶数之和，显然只要不等于2的所有偶数都符合条件
fn cf_4a_watermelon<R, W>(mut reader: R, mut writer: W) -> Result<(), Box<dyn std::error::Error>>
where
    R: std::io::BufRead,
    W: std::io::Write,
{
    let mut input = String::with_capacity(5);
    reader.read_line(&mut input)?;
    let num = input.trim_end().parse::<u8>()?;
    if num % 2 == 0 && num != 2 {
        write!(writer, "YES")?;
    } else {
        write!(writer, "NO")?;
    };
    Ok(())
}

#[test]
fn test_cf_4a_watermelon() {
    const TEST_CASES: [(&[u8], &[u8]); 2] = [
        (b"8\r\n", b"YES"), // codeforces testcase_1
        (b"99\n", b"NO"),   // mac_os input(without CR byte)
    ];
    for (input, expected_output) in TEST_CASES {
        let mut output = Vec::new();
        cf_4a_watermelon(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/** https://codeforces.com/problemset/problem/71/A
在英语中有一种将很长的单词缩写方法是: 首字母+中间有几个字母+尾字母
例如 kubernetes 开头k结尾s，k和s中间有8个字母，所以缩写成k8s
例如 internationalization 开头i结尾n中间18个字母，缩写成i18n
*/
fn cf_71a_way_too_long_words(
    reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines().flatten() {
        input.push(line);
    }
    for string in input.into_iter().skip(1) {
        let len = string.len();
        if len <= 10 {
            writeln!(writer, "{}", string)?;
        } else {
            let bytes = string.into_bytes();
            writeln!(
                writer,
                "{}{}{}",
                bytes[0] as char,
                len - 2, // len - 2(first and last)
                *bytes.last().unwrap() as char
            )?;
        }
    }
    Ok(())
}

#[test]
fn test_cf_71a_way_too_long_words() {
    const TEST_CASES: [(&[u8], &[u8]); 1] = [(
        b"4\nword\nlocalization\ninternationalization\npneumonoultramicroscopicsilicovolcanoconiosis",
        b"word\nl10n\ni18n\np43s\n",
    )];
    for (input, expected_output) in TEST_CASES {
        let mut output = Vec::new();
        cf_71a_way_too_long_words(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/// https://codeforces.com/problemset/problem/231/A
fn cf_231a_team(
    reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines().flatten() {
        input.push(line);
    }
    let mut ret = 0_u16;
    for each in input.into_iter().skip(1) {
        let each = each.into_bytes();
        let num_sure_about_the_solution = each[0] - b'0' + each[2] - b'0' + each[4] - b'0';
        if num_sure_about_the_solution >= 2 {
            ret += 1;
        }
    }
    write!(writer, "{}", ret)?;
    Ok(())
}

#[test]
fn test_cf_231a_team() {
    const TEST_CASES: [(&[u8], &[u8]); 1] = [(b"2\n1 0 0\n0 1 1\n", b"1")];
    for (input, expected_output) in TEST_CASES {
        let mut output = Vec::new();
        cf_231a_team(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

/// https://codeforces.com/problemset/problem/158/A
fn cf_158a_next_round(
    reader: impl std::io::BufRead,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines().flatten() {
        input.push(line);
    }
    let mut input_line1 = input[0].split_whitespace();
    let _n = input_line1.next().unwrap().parse::<u8>()?;
    let k = input_line1.next().unwrap().parse::<u8>()?;
    /*
    warning: found call to `str::trim_end` before `str::split_whitespace`
       --> src/easy/codeforces_easy.rs:163:10
        |
    163 |           .trim_end()
        |  __________^
    164 | |         .split_whitespace()
        | |_________^ help: remove `trim_end()`
        */
    let nums: Vec<u8> = input[1]
        // .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let nums_max = nums[0] as usize;
    let mut counter = [0_u8; 101];
    for num in nums {
        counter[num as usize] += 1;
    }
    let mut ret = 0_u8;
    // test_case: 4,2,[0,0,0,0], ignore counter[i] == 0
    for i in (1..=nums_max).rev() {
        if ret >= k {
            break;
        }
        ret += counter[i];
    }

    write!(writer, "{}", ret)?;
    Ok(())
}

#[test]
fn test_cf_158a_next_round() {
    const TEST_CASES: [(&[u8], &[u8]); 4] = [
        (b"17 14\n16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0\n", b"14"),
        (b"8 5\n10 9 8 7 7 7 5 5\n", b"6"),
        (b"4 2\n0 0 0 0\n", b"0"),
        (b"5 1\n1 1 1 1 1\n", b"5"),
    ];
    for (input, expected_output) in TEST_CASES {
        let mut output = Vec::new();
        cf_158a_next_round(input, &mut output).unwrap();
        assert_eq!(output, expected_output);
    }
}

fn main() {
    cf_158a_next_round(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
}
