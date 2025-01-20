pub fn urljoin(args: &[&str]) -> String {
    args.iter()
        .map(|s| s.trim_matches('/'))  // Strip leading and trailing slashes
        .collect::<Vec<&str>>()
        .join("/")  // Join with single slash
}
