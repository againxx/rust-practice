use regex::Regex;
use lazy_static::lazy_static;

const TO_SEARCH: &'static str = "
On 2017-12-31, happy. On 2018-01-01, New Year.
";

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?x)
        (?P<year>\d{4})- # the year
        (?P<month>\d{4})- # the year
        (?P<day>\d{2}) # the year
    ").unwrap();
    static ref EMAIL_RE: Regex = Regex::new(r"(?x)
        ^\w+@(?:gamil|163|qq)\.(?:com|cn|com\.cn|net)$
    ").unwrap();
}

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH) {
        println!("year: {}, month: {}, day: {}",
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str(),
        caps.get(3).unwrap().as_str());
    }
}
