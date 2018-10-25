extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Serialize;

/// Client object to connect to Elasticsearch REST interface.
/// Object stores Hyper Client instance
pub struct Client {
	client: reqwest::Client,
	host: String
}

impl Client {
	pub fn new(host: String) -> Client {
		Client {
			client: reqwest::Client::new(),
			host: host
		}
	}
}

/// Elasticsearch index object
pub struct Index {
	name: String
}

impl Index {
	/// Create new index object with client and index name.
	/// This does not result in index creation
	pub fn new(name: String) -> Index {
		Index {
			name: name,
		}
	}
	/// Index an object to create Elasticsearch document
	pub fn index<T: Serialize>(self, client: &Client, doc: T) {
		let url = format!("{}/{}/_doc", client.host, self.name);
		client.client.put(&url)
			.body(serde_json::to_string(&doc).unwrap())
			.send().unwrap();
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
