use dotenv::dotenv;
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
    let alphabets = "abcdefghijklnmopqrstuvwxyz".as_bytes();

    let mut string_arr: Vec<String> = vec![];

    for a in alphabets {
        for b in alphabets {
            for c in alphabets {
                let mut str = String::new();
                str.push(a.clone() as char);
                str.push(b.clone() as char);
                str.push(c.clone() as char);
                string_arr.push(str);
            }
        }
    }

    string_arr
}

async fn check_domain_available(
    api_key: &str,
    domain: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking {}.kr", domain);

    let url = Url::parse_with_params(
        "http://apis.data.go.kr/B551505/whois/domain_name?answer=json",
        &[
            ("serviceKey", api_key),
            ("query", &format!("{}.kr", domain)),
        ],
    )?;

    let resp = reqwest::get(url).await?.text().await?;
    let from_str = serde_json::from_str(&resp);

    if from_str.is_err() {
        println!("{}", &resp);
        Err("Couldn't parse data.")?
    }
    let parsed: Value = from_str.unwrap();

    let result_code = &parsed["response"]["result"]["result_code"];
    if *result_code == Value::Null {
        println!("The data seems to be wrong: {}", resp);
        Err("Wrong data")?
    } else {
        let result_code = result_code.as_str().unwrap_or("000");
        let result_code: i32 = FromStr::from_str(result_code)?;

        match result_code {
            0 => Err("result_code was null")?,
            22 => Err("The api key seems to be wrong")?,
            100 => Ok(false),
            113 => Ok(false), // 상기 도메인이름은 도메인이름의 안정적 관리와 공공의 이익 등을 위하여등록이 제한된 도메인이름입니다
            10000 => Ok(true),
            _ => {
                println!("{}", &resp);
                Err(format!(
                    "dunno about this type of result code: {}",
                    result_code
                ))?
            }
        }
    }
}
