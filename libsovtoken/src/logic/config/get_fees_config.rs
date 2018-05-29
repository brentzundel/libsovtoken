/*!
 *  Defines structure and implementation Fees and SetFeesRequest
 *  these are the structures for the [`build_fees_txn_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
use std::collections::HashMap;
use logic::request::Request;
use rand;
const GET_FEES : String = String::from("20001");

/**
 *  Json config to customize [`build_fees_txn_handler`]
 *
 *  [`build_fees_txn_handler`]: ../../../api/fn.build_fees_txn_handler.html
 */
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct getFeesRequest {
    #[serde(rename = "type")]
    txn_type: String,
}

impl getFeesRequest {

    pub fn new(identifiers : String, identifier: String) -> Request<getFeesRequest> {
        let req = getFeesRequest {
            txn_type: GET_FEES,
        };
        return Request::new(req, identifier);
    }
}