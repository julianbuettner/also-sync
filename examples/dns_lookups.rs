use std::{
    collections::HashMap,
};

use also_sync_macros::also_sync_tokio;
use async_dns::{lookup, AddressInfo};
use tokio::task;

struct DnsUplooker {
    targets: Vec<String>,
    results: Vec<task::JoinHandle<Result<Vec<AddressInfo>, std::io::Error>>>,
}

impl DnsUplooker {
    pub fn new(targets: Vec<String>) -> Self {
        Self {
            targets,
            results: Vec::new(),
        }
    }

    #[also_sync_tokio]
    pub async fn start_looking_up(&mut self) {
        let new = self
            .targets
            .clone()
            .into_iter()
            .map(|t| {
                task::spawn(
                    async move { lookup(t.as_str()).await.map(|val| val.collect::<Vec<_>>()) },
                )
            })
            .collect::<Vec<_>>();
        self.results = new;
    }

    #[also_sync_tokio]
    pub async fn get_output(&mut self) -> Result<HashMap<String, AddressInfo>, std::io::Error> {
        let mut result = HashMap::new();
        for (element, target) in self.results.drain(0..).zip(self.targets.iter()) {
            for sub in element.await?? {
                result.insert(target.clone(), sub);
            }
        }
        Ok(result)
    }
}

fn main() {
    let mut resolver = DnsUplooker::new(vec![
        "docs.rs".to_string(),
        "crates.io".to_string(),
        "rust-lang.org".to_string(),
    ]);

    resolver.start_looking_up_sync();
    // The tasks are now running in the global runtime
    match resolver.get_output_sync() {
        Err(e) => println!("Error: {}", e),
        Ok(m) => {
            for (item, value) in m.iter() {
                println!("{}: {}", item, value.ip_address)
            }
        }
    }
}
