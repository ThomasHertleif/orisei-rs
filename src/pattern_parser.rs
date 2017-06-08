#[test]
fn count_test() {
    let pat: Pattern = "Foo-{count}-".parse().unwrap();
    // let result = pat.fill(0).to_string();
    // assert_eq!("Foo-0-", result);
}

struct Pattern {
    number: i32,
    before: String,
    after: String,
}

#[derive(Debug)]
struct InvalidPattern;

impl ::std::str::FromStr for Pattern {
    type Err = InvalidPattern;

    fn from_str(input: &str) -> Result<Self, Self::Err>{
        let mut stuff = input.split("{count}");

        let pat = Pattern {
            number: 0,
            before: stuff.next().unwrap_or("").to_string(),
            after: stuff.next().unwrap_or("").to_string(),
        };

        Ok(pat)
    }
}
