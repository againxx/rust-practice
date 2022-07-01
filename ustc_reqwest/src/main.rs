use anyhow::anyhow;
use anyhow::Result;
use image::Luma;
use reqwest::cookie::{CookieStore, Jar};
use scraper::{Html, Selector};
use std::io::Read;
use std::sync::Arc;

const PASSPORT_URL: &str = "https://passport.ustc.edu.cn/login";
const CAS_URL: &str = "https://passport.ustc.edu.cn/login?service=https%3A%2F%2Fweixine.ustc.edu.cn%2F2020%2Fcaslogin";
const VALIDATE_IMAGE_URL: &str = "https://passport.ustc.edu.cn/validatecode.jsp?type=login";
const CLOCK_IN_URL: &str = "https://weixine.ustc.edu.cn/2020/daliy_report";

struct USTCPassportLogin {
    client: reqwest::Client,
    cookie_store: Arc<Jar>,
}

impl USTCPassportLogin {
    fn new() -> Self {
        let mut buf = Vec::new();
        std::fs::File::open("cacert.der").unwrap().read_to_end(&mut buf).unwrap();
        let cert = reqwest::Certificate::from_der(&buf).unwrap();

        let cookie_store = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36")
            .add_root_certificate(cert)
            .build()
            .unwrap();
        Self {
            client,
            cookie_store,
        }
    }

    async fn get_cas_lt(&self) -> Result<String> {
        let body = self.client.get(PASSPORT_URL).send().await?.text().await?;
        let document = Html::parse_document(&body);
        let cas_lt_selector =
            Selector::parse("#CAS_LT").map_err(|_| anyhow!("Selector parsing error!"))?;
        let cas_lt = document.select(&cas_lt_selector).next().unwrap();
        Ok(cas_lt
            .value()
            .attr("value")
            .ok_or_else(|| anyhow!("Unable to find CAS_LT!"))?
            .to_owned())
    }

    async fn get_validation_code(&self) -> Result<String> {
        let image_bytes = self.client.get(VALIDATE_IMAGE_URL)
            .send()
            .await?
            .bytes()
            .await?;
        let validation_image = image::load_from_memory(&image_bytes).unwrap();
        let image_luma = imageproc::map::map_colors(&validation_image, |p| {
            if p[1] as u32 > (p[0] as u32 + p[2] as u32 + 20) {
                Luma([0])
            } else {
                Luma([255])
            }
        });
        let dimension = image_luma.dimensions();
        let content = image_luma.into_vec();
        image::save_buffer("test3.jpeg", &content, dimension.0, dimension.1, image::ColorType::L8)?;
        let mut tess = tesseract::Tesseract::new(Some("data"), Some("digits"))
            .unwrap()
            .set_frame(
                &content,
                dimension.0 as i32,
                dimension.1 as i32,
                1,
                dimension.0 as i32,
            )?;
        Ok(tess.get_text()?.trim().to_owned())
    }

    async fn login(&self, user: &str, password: &str) -> Result<()> {
        let cas_lt = self.get_cas_lt().await?;
        let validation_code = self.get_validation_code().await?;
        let login_data = [
            ("model", "uplogin.jsp"),
            ("CAS_LT", &cas_lt),
            ("service", ""),
            ("warn", ""),
            ("showCode", "1"),
            ("qrcode", ""),
            ("username", user),
            ("password", password),
            ("LT", &validation_code),
            ("button", ""),
        ];
        let response = self
            .client
            .post(PASSPORT_URL)
            .form(&login_data)
            .send()
            .await?;

        let cookies = self.cookie_store.cookies(&PASSPORT_URL.parse()?);
        let login_err = || Err(anyhow!("Login failed!"));
        println!("{:?}", cookies);
        cookies.map_or_else(login_err, |header| {
            header
                .to_str()?
                .find("logins")
                .map_or_else(login_err, |_| Ok(()))
        })
    }

    async fn get_token(&self) -> Result<String> {
        let body = self.client.get(CAS_URL).send().await?.text().await?;
        let document = Html::parse_document(&body);
        let token_selector = Selector::parse(r#"input[name=_token]"#)
            .map_err(|_| anyhow!("Selector parsing error!"))?;
        let token = document.select(&token_selector).next().unwrap();
        Ok(token
            .value()
            .attr("value")
            .ok_or_else(|| anyhow!("Unable to find _token!"))?
            .to_owned())
    }

    async fn daily_clock_in(&self, post_data: &serde_json::Value) -> Result<()> {
        let response = self.client.post(CLOCK_IN_URL).json(post_data).send().await?;
        dbg!(response.text().await?.find("成功"));
        Ok(())
    }
}

fn read_password_file(path: &str) -> Result<(String, String)> {
    let content = std::fs::read_to_string(path)?;
    let (user, password) = content.trim().split_once(' ').unwrap();
    Ok((user.to_owned(), password.to_owned()))
}

fn read_clock_in_file(path: &str) -> Result<serde_json::Value> {
    let reader = std::fs::File::open(path)?;
    Ok(serde_json::from_reader::<_, serde_json::Value>(reader)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let password_file = args
        .next()
        .ok_or_else(|| anyhow!("Forget to provide the password file!"))?;
    let (user, password) = read_password_file(&password_file)?;
    let clock_in_file = args
        .next()
        .ok_or_else(|| anyhow!("Forget to provide the clock in file!"))?;
    let mut clock_in_data = read_clock_in_file(&clock_in_file)?;

    let loginer = USTCPassportLogin::new();
    loginer.login(&user, &password).await?;

    let token = loginer.get_token().await?;
    clock_in_data.as_object_mut().unwrap().insert("_token".to_owned(), token.into());
    println!("{:?}", clock_in_data);
    loginer.daily_clock_in(&clock_in_data).await?;

    Ok(())
}
