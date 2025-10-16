use crate::{evaluator::eval, lexer::Lexer, parser::Parser};
use nu_ansi_term::{Color, Style};
use reedline::{
    DefaultHinter, DefaultPrompt, DefaultPromptSegment, Emacs, FileBackedHistory, Highlighter,
    Reedline, Signal, StyledText,
};
use std::collections::HashSet;

const HISTORY_FILE: &str = ".monkey_history";

// ---- Prompt：支持续行 ----
fn base_prompt() -> DefaultPrompt {
    DefaultPrompt::new(
        DefaultPromptSegment::Basic("🦀".to_string()),
        DefaultPromptSegment::Empty,
    )
}
fn cont_prompt() -> DefaultPrompt {
    DefaultPrompt::new(
        DefaultPromptSegment::Basic("...".to_string()),
        DefaultPromptSegment::Empty,
    )
}

// ---- Darcula palette ----
fn c_rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}
fn style_keyword() -> Style {
    Style::new().fg(c_rgb(0xCC, 0x78, 0x32)).bold()
}
fn style_string() -> Style {
    Style::new().fg(c_rgb(0x6A, 0x87, 0x59))
}
fn style_number() -> Style {
    Style::new().fg(c_rgb(0x68, 0x97, 0xBB))
}
fn style_comment() -> Style {
    Style::new().fg(c_rgb(0x80, 0x80, 0x80)).italic()
}
fn style_symbol() -> Style {
    Style::new().fg(c_rgb(0xBB, 0xB5, 0x29))
}
fn style_default() -> Style {
    Style::new().fg(c_rgb(0xA9, 0xB7, 0xC6))
}
fn style_unmatched() -> Style {
    Style::new().fg(Color::Red).bold()
}

// ---- 实时高亮 + 括号匹配（未匹配标红） ----
struct DarculaHighlighter;

impl Highlighter for DarculaHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut out = StyledText::new();
        let mut buf = String::new();
        let mut in_string = false;
        let mut string_delim: Option<char> = None;

        let kw = style_keyword();
        let num = style_number();
        let sym = style_symbol();
        let stry = style_string();
        let cmt = style_comment();
        let norm = style_default();

        // 预扫描：未匹配括号（忽略字符串与 // 注释）
        let unmatched = find_unmatched_brackets(line);

        // 行内注释优先
        if let Some(i) = line.find("//") {
            let (code, comment) = line.split_at(i);
            flush_code_segment(
                code,
                &mut out,
                &mut buf,
                &mut in_string,
                &mut string_delim,
                &kw,
                &num,
                &sym,
                &stry,
                &norm,
                &unmatched,
            );
            out.push((cmt, comment.to_string()));
            return out;
        }

        flush_code_segment(
            line,
            &mut out,
            &mut buf,
            &mut in_string,
            &mut string_delim,
            &kw,
            &num,
            &sym,
            &stry,
            &norm,
            &unmatched,
        );
        out
    }
}

fn is_sym(c: char) -> bool {
    matches!(
        c,
        '(' | ')'
            | '{'
            | '}'
            | '['
            | ']'
            | ','
            | ':'
            | ';'
            | '='
            | '+'
            | '-'
            | '*'
            | '/'
            | '%'
            | '!'
            | '<'
            | '>'
            | '|'
            | '&'
            | '^'
    )
}

fn flush_word_token(tok: &str, out: &mut StyledText, kw: &Style, num: &Style, norm: &Style) {
    if tok.is_empty() {
        return;
    }
    if tok.chars().all(|c| c.is_ascii_digit()) {
        out.push((*num, tok.to_string()));
    } else if ["let", "fn", "if", "else", "return", "true", "false"].contains(&tok) {
        out.push((*kw, tok.to_string()));
    } else {
        out.push((*norm, tok.to_string()));
    }
}

// 预扫描未匹配括号（忽略字符串 & 注释）
fn find_unmatched_brackets(s: &str) -> HashSet<usize> {
    let mut unmatched = HashSet::new();
    let mut stack: Vec<(char, usize)> = Vec::new();
    let mut chars: Vec<char> = s.chars().collect();

    // 截断注释
    if let Some(i) = s.find("//") {
        chars.truncate(i);
    }

    // 标记字符串区域
    let mut mask = vec![false; chars.len()];
    let mut in_str = false;
    let mut delim: Option<char> = None;
    for (i, &ch) in chars.iter().enumerate() {
        if in_str {
            if Some(ch) == delim {
                in_str = false;
                delim = None;
            }
            mask[i] = true;
        } else if ch == '"' || ch == '\'' {
            in_str = true;
            delim = Some(ch);
            mask[i] = true;
        }
    }

    for (i, &ch) in chars.iter().enumerate() {
        if mask[i] {
            continue;
        }
        match ch {
            '(' | '{' | '[' => stack.push((ch, i)),
            ')' | '}' | ']' => match (stack.pop(), ch) {
                (Some(('(', _)), ')') => {}
                (Some(('{', _)), '}') => {}
                (Some(('[', _)), ']') => {}
                (Some((_, open_idx)), _) => {
                    unmatched.insert(open_idx);
                    unmatched.insert(i);
                }
                (None, _) => {
                    unmatched.insert(i);
                }
            },
            _ => {}
        }
    }
    for (_, idx) in stack {
        unmatched.insert(idx);
    }
    unmatched
}

fn flush_code_segment(
    s: &str,
    out: &mut StyledText,
    buf: &mut String,
    in_string: &mut bool,
    string_delim: &mut Option<char>,
    kw: &Style,
    num: &Style,
    sym: &Style,
    stry: &Style,
    norm: &Style,
    unmatched: &HashSet<usize>,
) {
    // 用字符索引判定括号是否未匹配
    let chars: Vec<char> = s.chars().collect();
    let mut idx = 0usize;

    while idx < chars.len() {
        let ch = chars[idx];

        if *in_string {
            buf.push(ch);
            if Some(ch) == *string_delim {
                out.push((*stry, buf.clone()));
                buf.clear();
                *in_string = false;
                *string_delim = None;
            }
            idx += 1;
            continue;
        }

        match ch {
            '\'' | '"' => {
                if !buf.is_empty() {
                    flush_word_token(buf, out, kw, num, norm);
                    buf.clear();
                }
                *in_string = true;
                *string_delim = Some(ch);
                buf.push(ch);
            }
            c if c.is_ascii_whitespace() => {
                if !buf.is_empty() {
                    flush_word_token(buf, out, kw, num, norm);
                    buf.clear();
                }
                out.push((*norm, c.to_string()));
            }
            c if is_sym(c) => {
                if !buf.is_empty() {
                    flush_word_token(buf, out, kw, num, norm);
                    buf.clear();
                }
                let style =
                    if matches!(c, '(' | ')' | '{' | '}' | '[' | ']') && unmatched.contains(&idx) {
                        style_unmatched()
                    } else {
                        *sym
                    };
                out.push((style, c.to_string()));
            }
            _ => buf.push(ch),
        }
        idx += 1;
    }

    if !buf.is_empty() {
        if *in_string {
            out.push((*stry, buf.clone()));
        } else {
            flush_word_token(buf, out, kw, num, norm);
        }
        buf.clear();
    }
}

// ---- 多行配平（整段） ----
fn is_balanced(s: &str) -> bool {
    let (mut r, mut c, mut sq) = (0i32, 0i32, 0i32);
    let mut in_str = false;
    let mut delim: Option<char> = None;

    for ch in s.chars() {
        if in_str {
            if Some(ch) == delim {
                in_str = false;
                delim = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                in_str = true;
                delim = Some(ch);
            }
            '(' => r += 1,
            ')' => r -= 1,
            '{' => c += 1,
            '}' => c -= 1,
            '[' => sq += 1,
            ']' => sq -= 1,
            _ => {}
        }
    }
    r == 0 && c == 0 && sq == 0 && !in_str
}

fn print_parser_errors(errors: &Vec<String>) {
    for msg in errors {
        println!("\t{}\n", msg);
    }
}

// ---- 入口 ----
pub fn start() {
    let mut rl = Reedline::create()
        .with_highlighter(Box::new(DarculaHighlighter))
        .with_hinter(Box::new(
            DefaultHinter::default().with_style(style_comment()), // 灰色历史提示
        ))
        .with_edit_mode(Box::new(Emacs::new(
            reedline::default_emacs_keybindings(), // 0.43 需要传入 Keybindings
        )));

    // 历史：↑/↓ 可用
    let history = Box::new(
        FileBackedHistory::with_file(50_000, HISTORY_FILE.into())
            .expect("history file open failed"),
    );
    rl = rl.with_history(history);

    println!("🦀 Monkey REPL  (:q 退出)");
    let mut buffer = String::new();

    loop {
        // 根据是否在续行，切换提示符
        let prompt = if buffer.is_empty() {
            base_prompt()
        } else {
            cont_prompt()
        };

        match rl.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                let line = line.trim_end();
                if line.is_empty() {
                    continue;
                }
                if matches!(line, ":q" | ":quit" | ":exit") {
                    break;
                }

                buffer.push_str(line);
                buffer.push('\n');

                if !is_balanced(&buffer) {
                    // 未配平：继续读取下一行（续行提示符 ...）
                    continue;
                }

                let lexer = Lexer::new(buffer.trim());
                let mut parser = Parser::new(lexer);
                match parser.parse_program() {
                    Some(program) => {
                        if !parser.errors().is_empty() {
                            print_parser_errors(&parser.errors());
                        } else {
                            let evaluated = eval(&program);
                            println!("{}", evaluated.inspect());
                        }
                    }
                    None => eprintln!("parse error: program is None"),
                }
                buffer.clear();
            }
            Ok(Signal::CtrlC) => {
                // 清空当前行
                println!();
            }
            Ok(Signal::CtrlD) => break,
            Err(err) => {
                eprintln!("type error: {:?} :)", err)
            }
        }
    }

    let _ = rl.sync_history();
    println!("Bye. Thanks for using it");
}
