//trololol copy paste!
extern crate hyper;
extern crate hyper_rustls;
extern crate google_sheets4 as sheets4;
use sheets4::api::ValueRange;
use sheets4::{Result, Error};
use std::default::Default;
use sheets4::{Sheets, oauth2, hyper, hyper_rustls};
 
// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
let secret: oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = ValueRange::default();
 
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
             .value_input_option("amet.")
             .response_value_render_option("duo")
             .response_date_time_render_option("ipsum")
             .insert_data_option("gubergren")
             .include_values_in_response(true)
             .doit().await;
 
match result {
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
let r = hub.spreadsheets().developer_metadata_get(...).doit().await
let r = hub.spreadsheets().developer_metadata_search(...).doit().await
let r = hub.spreadsheets().sheets_copy_to(...).doit().await
let r = hub.spreadsheets().values_append(...).doit().await
let r = hub.spreadsheets().values_batch_clear(...).doit().await
let r = hub.spreadsheets().values_batch_clear_by_data_filter(...).doit().await
let r = hub.spreadsheets().values_batch_get(...).doit().await
let r = hub.spreadsheets().values_batch_get_by_data_filter(...).doit().await
let r = hub.spreadsheets().values_batch_update(...).doit().await
let r = hub.spreadsheets().values_batch_update_by_data_filter(...).doit().await
let r = hub.spreadsheets().values_clear(...).doit().await
let r = hub.spreadsheets().values_get(...).doit().await
let r = hub.spreadsheets().values_update(...).doit().await
let r = hub.spreadsheets().batch_update(...).doit().await
let r = hub.spreadsheets().create(...).doit().await
let r = hub.spreadsheets().get(...).doit().await
let r = hub.spreadsheets().get_by_data_filter(...).doit().await