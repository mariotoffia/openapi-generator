/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActionContainer {
    #[serde(rename = "action")]
    pub action: Option<Box<crate::models::Baz>>,
}

impl ActionContainer {
    pub fn new(action: Option<crate::models::Baz>) -> ActionContainer {
        ActionContainer {
            action: if let Some(x) = action {Some(Box::new(x))} else {None},
        }
    }
}


