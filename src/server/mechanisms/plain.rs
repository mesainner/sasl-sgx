use crate::server::{Validator, Response, Mechanism};
use crate::common::Identity;
use crate::secret;
use std::prelude::v1::*;
use std::string::String;

pub struct Plain<V: Validator<secret::Plain>> {
    validator: V,
}

impl<V: Validator<secret::Plain>> Plain<V> {
    pub fn new(validator: V) -> Plain<V> {
        Plain { validator: validator, }
    }
}

impl<V: Validator<secret::Plain>> Mechanism for Plain<V> {
    fn name(&self) -> &str {
        "PLAIN"
    }

    fn respond(&mut self, payload: &[u8]) -> Result<Response, String> {
        let mut sp = payload.split(|&b| b == 0);
        sp.next();
        let username = sp.next().ok_or_else(|| "no username specified".to_owned())?;
        let username = String::from_utf8(username.to_vec()).map_err(|_| "error decoding username")?;
        let password = sp.next().ok_or_else(|| "no password specified".to_owned())?;
        let password = String::from_utf8(password.to_vec()).map_err(|_| "error decoding password")?;
        let ident = Identity::Username(username);
        self.validator.validate(&ident, &secret::Plain(password))?;
        Ok(Response::Success(ident, Vec::new()))
    }
}
