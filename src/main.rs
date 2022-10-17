use dotenv::dotenv;
use miette::{bail, IntoDiagnostic};
use reqwest::Url;
use serde_json::Value;
use std::{env, str::FromStr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("KRTLD_KEY")
        .expect("The environment `KRTLD_KEY` does not in the current environment.");

    let index = env::var("KRTLD_INDEX")
        .expect("The environment `KRTLD_INDEX` does not in the current environment.");

    let mut index: usize = index
        .trim()
        .parse()
        .expect(&format!("The index is not a number, found: {}", index));

    let arr = generate_arr();

    println!("Starting before of {}th element of vector.", index);

    for name in arr.into_iter().skip(index) {
        index += 1;
        let check = check_domain_available(&api_key, &name)
            .await
            .expect(&format!(
                "Failed while checking the domain status: {}, index {}",
                name, index
            ));
        if check {
            println!("Available: {}", name);
        }
    }
    println!("Done!");
}

fn generate_arr() -> Vec<String> {
    let alphabets = 'a'..='z';

    let mut string_arr: Vec<String> = vec![];

    for a in alphabets {
        for b in alphabets {
            for c in alphabets {
                string_arr.push(format!("{a}{b}{c}"));
            }
        }
    }

    string_arr
}

async fn check_domain_available(api_key: &str, domain: &str) -> miette::Result<bool> {
    println!("Checking {}.kr", domain);

    let url = Url::parse_with_params(
        "http://apis.data.go.kr/B551505/whois/domain_name?answer=json",
        &[
            ("serviceKey", api_key),
            ("query", &format!("{}.kr", domain)),
        ],
    )
    .into_diagnostic()?;

    let resp = reqwest::get(url)
        .await
        .into_diagnostic()?
        .text()
        .await
        .into_diagnostic()?;
    let from_str = serde_json::from_str(&resp);

    let parsed: Value = from_str.or_else(|_| {
        eprintln!("{}", &resp);
        bail!("Could not parse data.")
    })?;

    let result_code = &parsed["response"]["result"]["result_code"];
    if *result_code == Value::Null {
        eprintln!("The data seems to be wrong: {}", resp);
        bail!("Wrong Data")
    } else {
        let result_code = result_code.as_str().unwrap_or("000");
        let result_code = i32::from_str(result_code).into_diagnostic()?;

        match result_code {
            0 => bail!("result_code was null"),
            22 => bail!("The api key seems to be wrong"),
            100 => Ok(false),
            113 => Ok(false), // 상기 도메인이름은 도메인이름의 안정적 관리와 공공의 이익 등을 위하여등록이 제한된 도메인이름입니다
            10000 => Ok(true),
            _ => {
                println!("{}", &resp);
                bail!("dunno about this type of result code: {}", result_code)
            }
        }
    }
}
