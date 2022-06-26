//trololol copy paste!
extern crate hyper;
use hyper_rustls::HttpsConnectorBuilder;
use google_sheets4 as sheets4;
use sheets4::api::ValueRange;
use sheets4::{Result, Error};
use std::default::Default;
use sheets4::{Sheets, oauth2};
 
pub async fn test(){
    let str_vec_thing: Vec<String> = Vec::new();

    // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
    // `client_secret`, among other things.
    let secret: oauth2::ApplicationSecret = oauth2::ApplicationSecret{ //get from parking_service_account.json
        client_id: "196040464431-e3pdrk7356gl7je2s2o93ucv20mn26ln.apps.googleusercontent.com".to_string(), 
        client_secret: "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".to_string(), 
        token_uri: "https://oauth2.googleapis.com/token".to_string(), 
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(), 
        redirect_uris: str_vec_thing, 
        project_id: Some("crypto-arcade-350016".to_string()), 
        client_email: Some("parking-lab@crypto-arcade-350016.iam.gserviceaccount.com".to_string()), 
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()), 
        client_x509_cert_url: Some("https://www.googleapis.com/robot/v1/metadata/x509/parking-lab%40crypto-arcade-350016.iam.gserviceaccount.com".to_string())
    };
    // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        ).build().await.unwrap();
    let hub = Sheets::new(hyper::Client::builder()
        .build(HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build()
        ), auth
    );
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    //let req = ValueRange::default();
    
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    // let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
    //             .value_input_option("amet.")
    //             .response_value_render_option("duo")
    //             .response_date_time_render_option("ipsum")
    //             .insert_data_option("gubergren")
    //             .include_values_in_response(true)
    //             .doit().await;
    let r = hub.spreadsheets().get("CSUS Parking Data").add_ranges("A2Z500").doit().await;
    
    match r {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            |Error::Io(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }

    //more examples
    /*
    let r = hub.spreadsheets().developer_metadata_get(...).doit().await;
    let r = hub.spreadsheets().developer_metadata_search(...).doit().await;
    let r = hub.spreadsheets().sheets_copy_to(...).doit().await;
    let r = hub.spreadsheets().values_append(...).doit().await;
    let r = hub.spreadsheets().values_batch_clear(...).doit().await;
    let r = hub.spreadsheets().values_batch_clear_by_data_filter(...).doit().await;
    let r = hub.spreadsheets().values_batch_get(...).doit().await;
    let r = hub.spreadsheets().values_batch_get_by_data_filter(...).doit().await;
    let r = hub.spreadsheets().values_batch_update(...).doit().await;
    let r = hub.spreadsheets().values_batch_update_by_data_filter(...).doit().await;
    let r = hub.spreadsheets().values_clear(...).doit().await;
    let r = hub.spreadsheets().values_get(...).doit().await;
    let r = hub.spreadsheets().values_update(...).doit().await;
    let r = hub.spreadsheets().batch_update(...).doit().await;
    let r = hub.spreadsheets().create(...).doit().await;
    let r = hub.spreadsheets().get(...).doit().await;
    let r = hub.spreadsheets().get_by_data_filter(...).doit().await;
    */

}