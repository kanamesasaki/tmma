pub fn split_blocks(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut header = false;

    while let Some(end) = input[start..].find("HEADER") {
        if end == 0 {
            start += 6;
            header = true;
        } else {
            if header {
                result.push(&input[start-6..start+end]);
                header = false;
            } else {
                result.push(&input[start..start+end]);
            }
            start += end;
        }
    }

    if header {
        result.push(&input[start-6..]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_blocks() {
        let data = 
r#"HEADER OPTIONS DATA
TITLE HEATED BAR SAMPLE PROBLEM
    OUTPUT = BAR.OUT
    MODEL = TEST
HEADER NODE DATA, SUB1
    10, 70.0, 0.006 $ record 1
    15, 70.0, 0.006 $ record 2
    20, 70.0, 0.006 $ record 3
    -99, -460., 0.0 $ record 4
HEADER CONDUCTOR DATA, SUB1
    1015, 10, 15, 0.00417 $ record 5
    1520, 15, 20, 0.00417 $ record 6
    -2099, 20, 99, 1.98E-15 $ record 7
HEADER CONTROL DATA, GLOBAL
    TIMEND = 1000.0, OUTPUT = 1.0 $ record 8
HEADER SOURCE DATA, SUB1
    10, 10.0*3.413/60.0 $ record 9
HEADER OPERATIONS
BUILD ALL
    CALL TRANSIENT $ record 10"#;

    let result = split_blocks(data);
    println!("{:?}", result);
    }
}