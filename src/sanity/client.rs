extern crate sanity;
use std::ops::{Deref, DerefMut};

use sanity::helpers::get_json;
use sanity::SanityConfig;

#[derive(Debug)]
pub struct SanityClient(SanityConfig);

impl Deref for SanityClient {
    type Target = SanityConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SanityClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SanityClient {
    pub fn new(project_id: String, data_set: String, token: String, cdn: bool) -> Self {
        let client = sanity::create(&project_id, &data_set, &token, cdn);
        SanityClient(client)
    }
}

pub trait Client {
    fn get_client(&mut self) -> &mut SanityConfig;
    fn query(&mut self);
}

impl Client for SanityClient {
    fn get_client(&mut self) -> &mut SanityConfig {
        &mut self.0
    }

    fn query(&mut self) {
        let res = self
            .get_client()
            .get(&String::from("*[_type == 'article']"));
        println!("{:?}", get_json(res.unwrap()));
    }
}
