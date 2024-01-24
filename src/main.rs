use std::error::Error;

use rand::{rngs::OsRng, RngCore};
use thirtyfour::{components::SelectElement, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let kruzici = vec![
        vec![
            String::from("oneChoiceResponseMap837385730AD2497EA28BEE2F8A3ED7EB1"),
            String::from("oneChoiceResponseMapE0F4FE89367B44CAB8A4E62918C607AA1"),
            String::from("oneChoiceResponseMapC3C2BB83623F4209A86A5444701F42E41"),
            String::from("oneChoiceResponseMap9E59589179FE4082B308A31D328B6A6C1"),
        ],
        vec![
            String::from("oneChoiceResponseMap6878F1D99FB74C1587FBE78E0A784AF61"),
            String::from("oneChoiceResponseMapAC0AA54AFF424032BEB52E27A54010DF1"),
            String::from("oneChoiceResponseMapFB5EF65F847949AF8A46EA480D7C62841"),
            String::from("oneChoiceResponseMapDAD09789625B436CA8F7AB076493B2301"),
            String::from("oneChoiceResponseMap56BDEC360E9C414C8A8BB15F93FAC73C1"),
        ],
        vec![
            String::from("oneChoiceResponseMap5FAF3EFA38CA4124B9D946F50B31E2851"),
            String::from("oneChoiceResponseMap28B7194055CA45C39756F9C5E2E819111"),
            String::from("oneChoiceResponseMap3D892EF7989643EF9C4F590245338EF41"),
            String::from("oneChoiceResponseMapBB066F57E0ED4328AFE44EFB3DCCD7E21"),
        ],
        vec![
            String::from("oneChoiceResponseMap1F0EFAF6FE3A44ACB6FAC494C79F561E1"),
            String::from("oneChoiceResponseMapC238F6144E6143EFB915AE02407BBCEF1"),
            String::from("oneChoiceResponseMap1BB24C4D262344D4A11C68DC33BA0D0C1"),
            String::from("oneChoiceResponseMapE2D9A49B95D541BE88DFA0FB0162A89E1"),
            String::from("oneChoiceResponseMap086E084A54644A7E9C8AA618405CF31A1"),
            String::from("oneChoiceResponseMap906F40584B3C444F8B45CE04BD02D8DE1"),
        ],
    ];
    let pravougaonici = [
        String::from("oneChoiceResponseMap81F9CEF169EE4EA2B8CD779B7362B980"),
        String::from("oneChoiceResponseMap430605AA223347BA8B488BDC9909F847"),
        String::from("oneChoiceResponseMap850731D15C954136B073AE460169B4C2"),
        String::from("oneChoiceResponseMap57F2343A76C54C75B64CEB39EF231024"),
        String::from("oneChoiceResponseMap6C8034E3D17246A084BA8FCCC040BFBE"),
        String::from("oneChoiceResponseMapBF9DD9B4188B4CEE8E8B662393FF30FA"),
        String::from("oneChoiceResponseMapD2DED1D31F5C43A6A20398D88233110F"),
    ];

    let caps = DesiredCapabilities::firefox();
    // let proxy = Proxy::Manual {
    //     ftp_proxy: None,
    //     http_proxy: None,
    //     ssl_proxy: None,
    //     socks_proxy: Some(String::from("127.0.0.1:9050")),
    //     socks_version: Some(5),
    //     socks_username: None,
    //     socks_password: None,
    //     no_proxy: None,
    // };
    // caps.set_proxy(proxy)?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver
        .goto("https://schools-go-digital.jrc.ec.europa.eu/self-assessment?s=dAUCAEZ&lang=sr")
        .await?;

    driver.query(By::ClassName("ea_ignore")).first().await?;
    let mut elem_button = driver.find(By::ClassName("ea_ignore")).await?;
    while elem_button.click().await.is_err() {}

    driver
        .query(By::ClassName("wt-ecl-message__close"))
        .first()
        .await?;
    elem_button = driver.find(By::ClassName("wt-ecl-message__close")).await?;
    while elem_button.click().await.is_err() {}

    let mut count = 0;

    loop {
        driver
            .query(By::Id("btn-introduction-next"))
            .first()
            .await?;
        elem_button = driver.find(By::Id("btn-introduction-next")).await?;
        while elem_button.click().await.is_err() {}

        for row in kruzici.iter() {
            driver.query(By::Id("btn-survey-next")).first().await?;
            elem_button = driver.find(By::Id("btn-survey-next")).await?;

            for item in row {
                //driver.execute("scroll(0, 100)", Vec::new()).await?;
                clickit(item.clone(), &driver).await?;
            }

            while elem_button.click().await.is_err() {}
        }

        selectit(pravougaonici[0].clone(), &driver, OsRng.next_u32() % 6 + 16).await?;
        selectit(pravougaonici[1].clone(), &driver, OsRng.next_u32() % 5 + 1).await?;
        selectit(pravougaonici[2].clone(), &driver, OsRng.next_u32() % 5 + 1).await?;
        selectit(pravougaonici[3].clone(), &driver, OsRng.next_u32() % 5 + 1).await?;
        selectit(pravougaonici[4].clone(), &driver, OsRng.next_u32() % 5 + 1).await?;
        selectit(pravougaonici[5].clone(), &driver, OsRng.next_u32() % 5 + 1).await?;
        selectit(pravougaonici[6].clone(), &driver, OsRng.next_u32() % 5 + 1).await?;

        elem_button = driver
            .find_all(By::ClassName("ecl-checkbox__label"))
            .await?
            .last()
            .unwrap()
            .clone();
        elem_button.click().await?;

        driver.query(By::Id("btn-survey-next")).first().await?;
        elem_button = driver.find(By::Id("btn-survey-next")).await?;
        while elem_button.click().await.is_err() {}

        driver.query(By::Id("btn-submit")).first().await?;
        elem_button = driver.find(By::Id("btn-submit")).await?;
        while elem_button.click().await.is_err() {}

        count += 1;
        println!("forms submitted: {count}");

        driver
            .goto("https://schools-go-digital.jrc.ec.europa.eu/self-assessment?s=dAUCAEZ&lang=sr")
            .await?;
    }
}

async fn clickit(
    destinacija: String,
    vozac: &WebDriver,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    vozac.query(By::Id(&destinacija)).first().await?;
    let elem_input = vozac.find(By::Id(&destinacija)).await?;
    let elem_label = elem_input.find(By::XPath("./..")).await?;
    while elem_label.click().await.is_err() {}

    Ok(())
}

async fn selectit(
    destinacija: String,
    vozac: &WebDriver,
    zelja: u32,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    vozac.query(By::Id(&destinacija)).first().await?;
    let elem_select = vozac.find(By::Id(&destinacija)).await?;
    let selecting = SelectElement::new(&elem_select).await?;
    while selecting.select_by_index(zelja).await.is_err() {}

    Ok(())
}
