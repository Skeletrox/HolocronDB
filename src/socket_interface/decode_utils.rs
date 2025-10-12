// Importing everything to keep the decode clean.
// TODO(@Skeletrox): Split into req_decoders and resp_decoders?
use crate::proto::*;
use crate::key_value_store::key_value_pair;
use prost::Message;
use super::socket_errors::{SocketError, ErrorKind};

pub fn parse_generic_request(request: &[u8]) -> Result<GenericRequest, SocketError> {
    match GenericRequest::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}

pub fn parse_ping_request(request: &[u8]) -> Result<PingRequest, SocketError> {
    match PingRequest::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}

pub fn parse_create_request(request: &[u8]) -> Result<CreateKvPairReq, SocketError> {
    match CreateKvPairReq::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}

pub fn parse_read_request(request: &[u8]) -> Result<ReadKvPairReq, SocketError> {
    match ReadKvPairReq::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}

fn parse_ping_response(payload: &[u8]) -> Result<String, SocketError> {
    match PingResponse::decode(payload) {
        Ok(v) => {
            Ok(v.ping_resp_message.to_string())
        },
        Err(e) => {
            Err(SocketError {
                kind_: ErrorKind::ParseError,
                context_: e.to_string()
            })
        }
    }
}

fn parse_create_response(payload: &[u8]) -> Result<String, SocketError> {
    match CreateKvPairResp::decode(payload) {
        Ok(v) => {
            if (v.success) {
                Ok("Successfully created pair!".to_string())
            } else {
                Ok("Key already exists!".to_string())
            }
        },
        Err(e) => {
            Err(SocketError {
                kind_: ErrorKind::ParseError,
                context_: e.to_string()
            })
        }
    }
    
}

fn parse_read_response(payload: &[u8]) -> Result<String, SocketError> {
    match ReadKvPairResp::decode(payload) {
        Ok(v) => {
            if v.success {
                match v.pair {
                    Some(p) => Ok(p.value),
                    None => Err(SocketError {
                        kind_: ErrorKind::ParseError,
                        context_: "No pair in response".to_string()
                     })
                }
            } else {
                Ok("Cannot find key!".to_string())
            }
        },
        Err(e) => {
            Err(SocketError {
                kind_: ErrorKind::ParseError,
                context_: e.to_string()
            })
        }
    }
    
}


pub fn parse_generic_response(response: &[u8]) -> Result<String, SocketError> {
    let parsed_response: GenericResponse;
    match GenericResponse::decode(response) {
        Ok(res) => parsed_response = res,
        Err(e) => return Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
    let returnable: String;
    let req_type = parsed_response.req_type();
    let payload = parsed_response.payload;
    match req_type {
        ReqType::Ping => {
            match parse_ping_response(&payload) {
                Ok(v) => returnable = v,
                Err(e) => return Err(e)
            };
        },
        ReqType::Create => {
            match parse_create_response(&payload) {
                Ok(v) => returnable = v,
                Err(e) => return Err(e)
            };
        },
        ReqType::Read => {
            match parse_read_response(&payload) {
                Ok(v) => returnable = v,
                Err(e) => return Err(e)
            };
        },
        _ => { 
            returnable = "I did not understand what the server said".to_string();
        }
    }
    Ok(returnable)
}


pub fn kvp_proto_to_kvp_rust(inp: KeyValuePair) -> key_value_pair::KeyValuePair {
    key_value_pair::KeyValuePair::new(
        &inp.key,
        &inp.value
    )
}
