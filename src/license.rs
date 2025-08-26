use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug)]
enum LicenseParts<'a> {
    License(&'a str),
    ParenthesesStart,
    ParenthesesEnd,
    Or,
    And,
}

#[derive(Debug)]
pub enum LicenseParsed<'a> {
    License(&'a str),
    Group(Vec<LicenseParsed<'a>>),
    Or,
    And,
}

fn eval_from_parsed<'a>(expr: &[LicenseParsed<'a>], allow_list: &[&str]) -> bool {
    let mut result = None;
    let mut current_op = None;

    let mut i = 0;
    while i < expr.len() {
        match &expr[i] {
            LicenseParsed::License(license) => {
                let val = allow_list.contains(license);

                result = match result {
                    None => Some(val),
                    Some(acc) => match current_op {
                        Some(LicenseParsed::And) => Some(acc && val),
                        Some(LicenseParsed::Or) => Some(acc || val),
                        None => Some(val),
                        _ => panic!("Unexpected operator"),
                    },
                };
                current_op = None;
            }
            LicenseParsed::Group(license_parseds) => {
                let val = eval_from_parsed(license_parseds, allow_list);
                result = match result {
                    None => Some(val),
                    Some(acc) => match current_op {
                        Some(LicenseParsed::And) => Some(acc && val),
                        Some(LicenseParsed::Or) => Some(acc || val),
                        None => Some(val),
                        _ => panic!("Unexpected operator"),
                    },
                };
                current_op = None;
            }
            LicenseParsed::And => {
                current_op = Some(LicenseParsed::And);
            }
            LicenseParsed::Or => {
                current_op = Some(LicenseParsed::Or);
            }
        }

        i += 1;
    }

    result.unwrap_or(false)
}

fn after_parse<'a>(
    output: &mut Vec<LicenseParsed<'a>>,
    before_parse: &[LicenseParts<'a>],
    start_idx: usize,
) -> usize {
    let mut i = start_idx;

    while i < before_parse.len() {
        match &before_parse[i] {
            LicenseParts::License(l) => {
                output.push(LicenseParsed::License(l));
                i += 1;
            }
            LicenseParts::ParenthesesStart => {
                let mut temp_parsed = vec![];
                i += 1;

                let consumed = after_parse(&mut temp_parsed, before_parse, i);
                i += consumed;

                output.push(LicenseParsed::Group(temp_parsed));

                i += 1;
            }
            LicenseParts::ParenthesesEnd => {
                return i - start_idx;
            }
            LicenseParts::Or => {
                output.push(LicenseParsed::Or);
                i += 1;
            }
            LicenseParts::And => {
                output.push(LicenseParsed::And);
                i += 1;
            }
        }
    }

    i - start_idx
}

fn split_license(license: &str) -> Vec<&str> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(\s*OR\s*|\s*AND\s*|\(|\)|\s*/\s*|/)").unwrap());
    let mut result = Vec::new();
    let mut last_end = 0;

    for mat in RE.find_iter(license) {
        let token = license[last_end..mat.start()].trim();
        if !token.is_empty() {
            result.push(token);
        }

        let delim = mat.as_str().trim();
        if !delim.is_empty() {
            result.push(delim);
        }

        last_end = mat.end();
    }

    let remainder = license[last_end..].trim();
    if !remainder.is_empty() {
        result.push(remainder);
    }
    result
}

fn before_parse<'a>(license: &'a str) -> Vec<LicenseParts<'a>> {
    let mut before_parse = vec![];

    split_license(license).into_iter().for_each(|i| match i {
        "(" => before_parse.push(LicenseParts::ParenthesesStart),
        ")" => before_parse.push(LicenseParts::ParenthesesEnd),
        "and" | "AND" => before_parse.push(LicenseParts::And),
        "or" | "OR" | "/" => before_parse.push(LicenseParts::Or),
        _ => before_parse.push(LicenseParts::License(i)),
    });

    before_parse
}

pub fn parse<'a>(license: &'a str) -> Vec<LicenseParsed<'a>> {
    let before_parsed = before_parse(license);

    let mut output = vec![];
    after_parse(&mut output, &before_parsed, 0);
    output
}

pub fn license_check(license: &str, allow_list: &[&str]) -> bool {
    let parsed = parse(license);
    eval_from_parsed(&parsed, allow_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn dbg_parse() {
        let license =
            "((MIT OR Apache-2.0) OR Unicode-3.0) / ( BSL-1.0 )/ Apache-2.0 WITH LLVM-exception";

        let before_parsed = before_parse(license);
        dbg!(&before_parsed);

        let mut output = vec![];
        after_parse(&mut output, &before_parsed, 0);
        dbg!(output);
    }

    #[test]
    fn test_1() {
        let license = "MIT OR Apache-2.0/Unicode-3.0 / BSL-1.0";
        let allow_list_1 = ["MIT"];
        let allow_list_2 = ["Apache-2.0"];
        let allow_list_3 = ["Unicode-3.0"];
        let allow_list_4 = ["BSL-1.0"];
        let allow_list_5 = ["MIT", "Apache-2.0"];

        assert!(license_check(license, &allow_list_1));
        assert!(license_check(license, &allow_list_2));
        assert!(license_check(license, &allow_list_3));
        assert!(license_check(license, &allow_list_4));
        assert!(license_check(license, &allow_list_5));
    }

    #[test]
    fn test_2() {
        let license = "MIT AND Apache-2.0 AND Unicode-3.0 AND BSL-1.0";
        let allow_list_1 = ["MIT", "Apache-2.0", "Unicode-3.0", "BSL-1.0"];
        let allow_list_2 = ["MIT"];
        let allow_list_3 = ["Apache-2.0", "Unicode-3.0", "BSL-1.0"];

        assert!(license_check(license, &allow_list_1));

        assert!(!license_check(license, &allow_list_2));
        assert!(!license_check(license, &allow_list_3));
    }

    #[test]
    fn test_3() {
        let license = "(MIT OR Apache-2.0) AND (Unicode-3.0 OR BSL-1.0)";
        let allow_list_1 = ["MIT", "Unicode-3.0"];
        let allow_list_2 = ["Apache-2.0", "BSL-1.0"];
        let allow_list_3 = ["Apache-2.0", "Unicode-3.0"];
        let allow_list_4 = ["MIT", "BSL-1.0"];
        let allow_list_5 = ["MIT"];
        let allow_list_6 = ["Apache-2.0"];
        let allow_list_7 = ["MIT", "Apache-2.0"];
        let allow_list_8 = ["Unicode-3.0"];
        let allow_list_9 = ["BSL-1.0"];
        let allow_list_10 = ["Unicode-3.0", "BSL-1.0"];

        assert!(license_check(license, &allow_list_1));
        assert!(license_check(license, &allow_list_2));
        assert!(license_check(license, &allow_list_3));
        assert!(license_check(license, &allow_list_4));

        assert!(!license_check(license, &allow_list_5));
        assert!(!license_check(license, &allow_list_6));
        assert!(!license_check(license, &allow_list_7));
        assert!(!license_check(license, &allow_list_8));
        assert!(!license_check(license, &allow_list_9));
        assert!(!license_check(license, &allow_list_10));
    }

    #[test]
    fn test_4() {
        let license = "((MIT AND Apache-2.0) OR Unicode-3.0) OR ( BSL-1.0)";
        let allow_list_1 = ["MIT", "Apache-2.0"];
        let allow_list_2 = ["Unicode-3.0"];
        let allow_list_3 = ["BSL-1.0"];

        let allow_list_4 = ["MIT"];
        let allow_list_5 = ["Apache-2.0"];

        assert!(license_check(license, &allow_list_1));
        assert!(license_check(license, &allow_list_2));
        assert!(license_check(license, &allow_list_3));

        assert!(!license_check(license, &allow_list_4));
        assert!(!license_check(license, &allow_list_5));
    }
}
