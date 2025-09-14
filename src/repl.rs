use crate::lexer::Lexer;
use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let handle = stdin.lock();

    // 第一次提示
    print!("{PROMPT}");
    stdout.flush().unwrap();

    for line in handle.lines() {
        match line {
            Ok(text) => {
                let src = text.trim_end(); // 去掉行尾 \n/\r\n
                if src.is_empty() {
                    // 空行：直接给下一次机会
                    print!("{PROMPT}");
                    stdout.flush().unwrap();
                    continue;
                }
                // 方便退出
                if matches!(src, ":q" | ":quit" | ":exit") {
                    break;
                }

                // 词法分析（你的 Lexer 实现了 Iterator）
                let lexer = Lexer::new(src);
                for tok in lexer {
                    println!("{:?}", tok);
                }
            }
            Err(e) => {
                eprintln!("输入错误：{e}");
                // 不退出，继续下一轮
            }
        }

        // 下一个提示
        print!("{PROMPT}");
        stdout.flush().unwrap();
    }

    println!("Bye. Thanks for using it");
}
