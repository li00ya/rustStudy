/*
题目描述：
	将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
L   C   I   R
E T O E S I I G
E   D   H   N
之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。

请你实现这个将字符串进行指定行数变换的函数：
string convert(string s, int numRows);
示例 1:
输入: s = "LEETCODEISHIRING", numRows = 3
输出: "LCIRETOESIIGEDHN"
示例 2:
输入: s = "LEETCODEISHIRING", numRows = 4
输出: "LDREOEIIECIHNTSG"
解释:
L     D     R
E   O E   I I
E C   I H   N
T     S     G
*/
fn convert(s: String, num_rows: i32) -> String
{
	let len = s.len();

	if num_rows < 2 || len <= 2 {
		return s;
	}

	let num_rows = num_rows as usize;
	let s = s.as_bytes();
	let mut st = String::new();
	let rows = num_rows - 1;
	let mut i:usize = 0;

	while i < num_rows {
		let gap = (rows - i) * 2;
		let mut j = i;
		while j < len {
			let c = s[j] as char;
			st.push(c);
			if i != 0 && i != rows && j + gap < len {
				let c = s[j + gap] as char;
				st.push(c);
			}
			j += rows * 2;
		}
		i += 1;
	}
	st
}

fn main()
{
	let s1 = "LEETCODEISHIRING".to_string();
	let s2 = s1.clone();
	let s3 = s1.clone();

	let st = convert(s1, 3);
	println!("ret1 {}", st);
	let st = convert1(s2, 3);
	println!("ret2 {}", st);
	let st = convert2(s3, 3);
	println!("ret3 {}", st);
}

/* 大神题解 */
/*耗时0ms*/
fn convert1(s: String, num_rows: i32) -> String
{        
        if num_rows == 1 { return s; }
        let mut result: Vec<String> = vec![];
        
        for _ in 0..num_rows{
            result.push("".to_string());
        }
        
        for (i, val) in s.chars().enumerate(){
            let index = i % (2 * num_rows - 2) as usize;
            if index < num_rows as usize{
                result[index].push(val); 
            }else{
                result[(2 * num_rows - 2) as usize - index].push(val);
            }
        }
        let mut ans = String::new();
            
        for val in &result {
            ans.push_str(val);
        }
            
        return ans;
}

/*消耗内存 2056kb*/
fn convert2(s: String, num_rows: i32) -> String
{
        if num_rows == 1 {
          return s;
        }
        let mut final_strings: Vec<String> = Vec::with_capacity(num_rows as usize);

        for _ in 0..num_rows {
          final_strings.push(String::new());
        }

        let mut is_reverse = false;
        let mut last_pos: usize = 0;

        for ch in s.as_bytes().iter() {
          if !is_reverse {
            final_strings[last_pos].push(*ch as char);
            if (last_pos as i32) == (num_rows - 1) {
              is_reverse = true;
              continue
            }
            last_pos = last_pos + 1;
            continue;
          }

          if is_reverse {
            last_pos = last_pos - 1;
            final_strings[last_pos].push(*ch as char);
            if last_pos == 0 {
              last_pos = last_pos + 1;
              is_reverse = false;
              continue
            }

            continue
          }
        }

        let mut final_str = String::new();

        for n in 0..num_rows {
          final_str.push_str(final_strings[n as usize].as_str());
        }

        final_str
}
