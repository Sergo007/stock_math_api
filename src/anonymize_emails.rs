pub fn anonymize_emails(s: &mut str) {
    // 's' should be only utf-8
    let re = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap();
    let matches: Vec<_> = re.find_iter(&s).map(|m| m.range()).collect();
    let s_bytes: &mut [u8] = unsafe { s.as_bytes_mut() };
    for range in matches {
        let replacement = vec![b'*'; range.end - range.start];
        s_bytes[range].copy_from_slice(&replacement);
    }
}

#[cfg(test)]
mod anonymize_emails_tests {
    use crate::anonymize_emails::anonymize_emails;

    #[test]
    fn anonymize_emails1() {
        let mut ss = "asd yatsyna.sergey@gmail.com fdsa".to_string();
        anonymize_emails(&mut ss);
        println!("{}", ss);
        assert_eq!(4, 4);
    }
}
