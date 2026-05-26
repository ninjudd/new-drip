pub fn inserted_lines(old: &str, new: &str) -> Vec<String> {
    let old_lines: Vec<&str> = old.lines().map(|l| l.trim()).collect();
    let new_lines: Vec<&str> = new.lines().map(|l| l.trim()).collect();

    let m = old_lines.len();
    let n = new_lines.len();
    let mut dp = vec![vec![0u32; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if old_lines[i - 1] == new_lines[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut inserted = Vec::new();
    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        if old_lines[i - 1] == new_lines[j - 1] {
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else if dp[i][j - 1] > dp[i - 1][j] {
            inserted.push(new_lines[j - 1].to_string());
            j -= 1;
        } else {
            // Modification — suppress both
            i -= 1;
            j -= 1;
        }
    }
    while j > 0 {
        inserted.push(new_lines[j - 1].to_string());
        j -= 1;
    }

    inserted.reverse();
    inserted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insertion() {
        let old = "line1\nline2";
        let new = "line1\nline2\nline3";
        assert_eq!(inserted_lines(old, new), vec!["line3"]);
    }

    #[test]
    fn modification_suppressed() {
        let old = "header\nstatus: 50%\nfooter";
        let new = "header\nstatus: 75%\nfooter";
        assert_eq!(inserted_lines(old, new), Vec::<String>::new());
    }

    #[test]
    fn trailing_whitespace_ignored() {
        let old = "hello  \nworld";
        let new = "hello\nworld\nnew line";
        assert_eq!(inserted_lines(old, new), vec!["new line"]);
    }

    #[test]
    fn whitespace_only_lines_match() {
        let old = "header\n   \nfooter";
        let new = "header\n\nfooter\nnew";
        assert_eq!(inserted_lines(old, new), vec!["new"]);
    }

    #[test]
    fn scrolling_does_not_duplicate() {
        let old = "line1\nline2\nline3\nCurrent state\nstatus bar";
        let new = "line3\nCurrent state\nThe working tree has changes.\nstatus bar";
        assert_eq!(inserted_lines(old, new), vec!["The working tree has changes."]);
    }

    #[test]
    fn scrolling_with_whitespace_variance() {
        let old = "line1\nline2\n \nCurrent state\nstatus";
        let new = "line2\n\nCurrent state\nnew content\nstatus";
        assert_eq!(inserted_lines(old, new), vec!["new content"]);
    }

    #[test]
    fn empty_old() {
        assert_eq!(inserted_lines("", "line1\nline2"), vec!["line1", "line2"]);
    }

    #[test]
    fn identical_screens() {
        assert_eq!(inserted_lines("a\nb\nc", "a\nb\nc"), Vec::<String>::new());
    }
}
