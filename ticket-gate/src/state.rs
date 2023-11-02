use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize, Serializer};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, time::SystemTime};
use tracing::{error, info};

use crate::docker;

const DEFAULT_POD_PRICE: u128 = 100;

#[derive(Default)]
pub struct State {
    pub users: HashMap<String, User>,
    pub pods: Vec<Pod>,
    next_pod_id: u128,
}

// Query methods
impl State {
    pub fn balance_of(&self, user: &str) -> Option<u128> {
        self.users.get(user).map(|user| user.balance)
    }

    pub fn pods_of(&self, user: &str) -> Vec<&Pod> {
        self.pods
            .iter()
            .filter(|pod| pod.owner == user)
            .collect::<Vec<_>>()
    }
}

impl State {
    pub fn recharge(&mut self, user: &str, amount: u128) {
        let user = self.users.entry(user.to_string()).or_default();
        user.balance += amount;
    }

    pub fn add_pod(
        &mut self,
        owner: String,
        image: String,
        cmd: String,
        name: String,
    ) -> Result<u128> {
        let name = format!("phat-pod-{owner}-{name}");
        let user = self
            .users
            .get_mut(&owner)
            .ok_or(anyhow!("User not found"))?;
        user.balance = user.balance
            .checked_sub(DEFAULT_POD_PRICE)
            .ok_or(anyhow!("Insufficient funds"))?;
        let pod = Pod::new(self.next_pod_id, name, DEFAULT_POD_PRICE, owner, image, cmd);
        self.pods.push(pod);
        self.next_pod_id += 1;
        Ok(self.next_pod_id - 1)
    }

    pub fn get_pod(&self, id: u128) -> Option<&Pod> {
        self.pods.iter().find(|pod| pod.id == id)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub balance: u128,
}

#[derive(Serialize, Clone)]
pub struct Pod {
    pub id: u128,
    pub price: u128,
    pub owner: String,
    pub image: String,
    #[serde(serialize_with = "serialize_state")]
    pub state: Arc<Mutex<PodState>>,
    pub container_name: String,
    pub cmd: String,
    pub last_paid: SystemTime,
}

#[derive(Serialize, Clone, Default, PartialEq, Eq, Debug)]
pub enum PodState {
    Starting,
    Running,
    Stopping,
    #[default]
    Stopped,
}

impl Pod {
    pub fn new(
        id: u128,
        name: String,
        price: u128,
        owner: String,
        image: String,
        cmd: String,
    ) -> Self {
        Self {
            id,
            price,
            owner,
            image,
            cmd,
            container_name: name,
            state: Default::default(),
            last_paid: SystemTime::now(),
        }
    }

    pub fn start(&self) {
        let state = self.state.clone();
        let container_name = self.container_name.clone();
        let image = self.image.clone();
        let cmd = self.cmd.clone();
        *state.lock().unwrap() = PodState::Starting;
        tokio::spawn(async move {
            let result = docker::start_container(&container_name, &image, Some(&cmd)).await;
            match result {
                Ok(id) => {
                    info!("Started container {container_name}, id={id}");
                    *state.lock().unwrap() = PodState::Running;
                }
                Err(err) => {
                    error!("Failed to start container: {err:?}");
                    *state.lock().unwrap() = PodState::Stopped;
                }
            }
        });
    }

    pub fn stop(&self) {
        let state = self.state.clone();
        let container_name = self.container_name.clone();
        *state.lock().unwrap() = PodState::Stopping;
        tokio::spawn(async move {
            if let Err(err) = docker::stop_container(&container_name).await {
                error!("Failed to stop container: {err:?}");
            }
            *state.lock().unwrap() = PodState::Stopped;
        });
    }
}

pub fn serialize_state<S>(value: &Arc<Mutex<PodState>>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.lock().unwrap().serialize(ser)
}
