use std::sync::{ Arc, Mutex, mpsc::{ channel, Sender, Receiver } };
use std::thread;
use std::ops::Deref;
use std::collections::HashMap;
use crate::configuration::{ Configuration, ServerID, ServerSuffrage };

#[derive(Clone)]
pub struct Commitment {
    pub commit_ch: Arc<Mutex<u64>>,
    pub match_indexes: HashMap<ServerID, u64>,
    pub commit_index: u64,
    pub start_index: u64
}

impl Commitment {
    pub fn new(
        commit_ch: Arc<Mutex<u64>>,
        configuration: Configuration,
        start_index: u64
    ) -> Self {

        let mut match_indexes = HashMap::new();

        for server in configuration.servers {
            if server.suffrage == ServerSuffrage::Voter {
                match_indexes.insert(server.id, 0);
            };
        }
        Commitment {
            commit_ch: commit_ch,
            match_indexes: match_indexes,
            commit_index: 0,
            start_index: start_index
        }
    }

    pub fn set_configuration(self, configuration: &'static Configuration) {
        const N: usize = 10;
        let data = Arc::new(Mutex::new(self));

        let (tx, rx): (Sender<Commitment>, Receiver<Commitment>) = channel();

        for _ in 0..N {
            let (data, tx) = (Arc::clone(&data), tx.clone());

            thread::spawn(move || {

                let mut data = data.lock().unwrap();

                let old_match_index = data.match_indexes.clone();
                data.match_indexes.clear();

                for server in configuration.clone().servers {
                    if server.suffrage == ServerSuffrage::Voter {
                        data.match_indexes.insert(server.id.clone(), *old_match_index.get(&server.id).unwrap());
                    }
                }
                
                data.clone().recalculate();
                tx.send(data.deref().clone()).unwrap();
            });
        }

        rx.try_recv().unwrap();
    }

    pub fn get_commit_index(self) -> u64 {
        const N: usize = 10;
        let data = Arc::new(Mutex::new(self));

        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();

        for _ in 0..N {
            let (data, tx) = (Arc::clone(&data), tx.clone());

            thread::spawn(move || {
                let data = data.lock().unwrap();
                tx.send(data.deref().clone().commit_index).unwrap();
            });
        }

        rx.try_recv().unwrap()
    }

    pub fn commitment_match<'a>(self, server: &'static ServerID, match_index: u64) {

        const N: usize = 10;
        let data = Arc::new(Mutex::new(self));

        let (tx, rx): (Sender<Commitment>, Receiver<Commitment>) = channel();

        for _ in 0..N {
            let (data, tx) = (Arc::clone(&data), tx.clone());

            thread::spawn(move || {

                let mut data = data.lock().unwrap();

                let prev = data.match_indexes.get(server).unwrap();
                let has_vote = data.match_indexes.get(server).unwrap();

                if has_vote > &0 && match_index > *prev {
                    data.match_indexes.insert(server.to_string(), match_index);
                    data.clone().recalculate();
                    tx.send(data.deref().clone()).unwrap();
                }
            });
        }

        rx.try_recv().unwrap();
    }

    pub fn recalculate(mut self) {
        if self.match_indexes.len() == 0 {
            return;
        }

        let mut matched: Vec<u64> = Vec::with_capacity(self.match_indexes.len());

        for idx in 0..self.match_indexes.len() {
            matched.push(idx.try_into().unwrap());
        }

        matched.sort();

        let quorum_match_index = matched[(matched.len() - 1)/2].clone();

        if quorum_match_index > self.commit_index && quorum_match_index >= self.start_index {
            self.commit_index = quorum_match_index;
        }
    }
}