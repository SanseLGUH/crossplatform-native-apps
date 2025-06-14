use tokio::sync::{Mutex, Notify};
use std::sync::Arc;

use druid::im::Vector;

use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

use tokio::task;

use crate::data::{DiscordRole, DiscordChannels, DiscordRolePayload, 
	ToggleSettings, ChannelPermission};

use std::collections::HashMap;

pub struct FetchingEvent {
	// Most important logic 
	channels_reference: HashMap<String, String>,
	roles_reference: HashMap<String, String>,

	// necessary data
	progress_data: Arc<Mutex<String>>, 
	notify: Arc<Notify>, 
	your_server_id: String, 
	target_server_id: String, 
	your_token_auth: String,
}

impl FetchingEvent {
	pub fn new(
		progress_data: Arc<Mutex<String>>, 
		notify: Arc<Notify>, 
		your_server_id: String, 
		target_server_id: String, 
		your_token_auth: String,
		) -> Self {

		FetchingEvent {
			channels_reference: HashMap::new(),
			roles_reference: HashMap::new(),
			progress_data, notify, 
			your_server_id, target_server_id,
			your_token_auth
		}
	}

	async fn take_data<X, P>(
		&self, method: Method, url: &str, payload: Option<P>) -> Option<X>
		where 
			X: DeserializeOwned + Default,
			P: Serialize + Default, {

		let client = Client::new();
		let mut request_builder = client.request(method.clone(), 
			format!("https://discord.com/api/v9{}", &*url));
		request_builder = request_builder.header("Authorization", self.your_token_auth.clone());

		if method == Method::POST {
			if let Some(payload) = payload {
				request_builder = request_builder.json(&payload);
			}
		}

		let response = match request_builder.send().await {
			Ok(resp) => resp,
			Err(e) => return None,
		};

	    let response_json: Value = match response.json().await {
	        Ok(json) => json,
	        Err(e) => {
	        	println!("{:?} <- Value?", e);
	        	return None;
	        },
	    };

	    println!("{:?}", response_json);

	    match serde_json::from_value::<X>(response_json) {
	        Ok(deserialized) => Some(deserialized),
	        Err(e) => {
	        	println!("{:?} <- From Value?", e);
	        	return None;
	     	}
	    }
	}

	pub async fn clean_up(&self) {
		let delete_role: Option<Vec<DiscordRole>> =
			self.take_data::<Vec<DiscordRole>, ()>(Method::GET, 
				&format!("/guilds/{}/roles", self.your_server_id), None).await;

		if let Some(roles) = delete_role {
			for role in roles {
				tokio::time::sleep(std::time::Duration::from_secs(1)).await;

				let deleted = self.take_data::<DiscordRole, ()>(Method::DELETE,
					&format!("/guilds/{}/roles/{}", self.your_server_id, role.id), None).await;
			}
		}

		let delete_channel: Option<Vec<DiscordChannels>> = 
			self.take_data::<Vec<DiscordChannels>, ()>(Method::GET, 
				&format!("/guilds/{}/channels", self.your_server_id), None).await;

		if let Some(channels) = delete_channel {
			for channel in channels {
				tokio::time::sleep(std::time::Duration::from_secs(1)).await;

				let deleted = self.take_data::<DiscordChannels, ()>(Method::DELETE, 
					&format!("/channels/{}", channel.id.unwrap()), None).await;

				let mut progress_data_lock = self.progress_data.lock().await;
				*progress_data_lock = format!("{:?}", deleted);
				self.notify.notify_one();
			}
		}
	}

	pub async fn copy_roles(&mut self) {
		tokio::time::sleep(std::time::Duration::from_secs(2)).await;

		let target_server_data: Option<Vec<DiscordRole>> = 
			self.take_data::<Vec<DiscordRole>, ()>(Method::GET, &format!("/guilds/{}/roles", 
				self.target_server_id), None).await;

		if let Some(server_data) = target_server_data {
			for role in server_data {

				tokio::time::sleep(std::time::Duration::from_secs(2)).await;

				let your_role = DiscordRolePayload {
					name: role.name,
					permissions: role.permissions,
					color: role.color,
				};

				let role_response: Option<DiscordRole> =
					self.take_data::<DiscordRole, DiscordRolePayload>(Method::POST, 
						&format!("/guilds/{}/roles", 
						self.your_server_id), Some(your_role.clone())).await;

				self.roles_reference.insert(role.id.clone().to_string(), 
					role_response.clone().unwrap().id.to_string());				

				let mut progress_data_lock = self.progress_data.lock().await;
				*progress_data_lock = format!("{:?}", role_response);
				self.notify.notify_one();
			}
		}
	}

	pub async fn copy_channels(&mut self, roles_params: bool) {
		let target_server_data: Option<Vec<DiscordChannels>> =
			self.take_data::<Vec<DiscordChannels>, ()>(Method::GET, 
				&format!("/guilds/{}/channels", 
				self.target_server_id), None).await;

		println!("[ {:?} ] self.roles_reference", self.roles_reference);

		let mut permission_overwrites = ChannelPermission::default(&self.your_server_id);

		if let Some(server_data) = target_server_data {
			for category in server_data.iter().filter(|category| category.r#type == 4) {
				tokio::time::sleep(std::time::Duration::from_secs(2)).await;

				if roles_params {
					for roles in category.permission_overwrites.iter().filter(|role| role.r#type == 0) {
						println!("{:?}", roles);
						permission_overwrites.push(
								ChannelPermission {
									allow: roles.allow.clone(),
									deny: roles.deny.clone(),
									id: self.roles_reference.get(&roles.id.clone()
										.to_string()).unwrap().to_string(),
									r#type: 0,
								}
							);
					}
				} 

				let payload = DiscordChannels {
					id: None,
					name: category.name.clone(),
					r#type: category.r#type,
					position: category.position,
					parent_id: None,
					permission_overwrites: permission_overwrites.clone(),
				};

				let your_category: Option<DiscordChannels> = 
					self.take_data::<DiscordChannels, DiscordChannels>(Method::POST, &format!("/guilds/{}/channels", 
						self.your_server_id), Some(payload)).await;
				
				let mut progress_data_lock = self.progress_data.lock().await;
				*progress_data_lock = format!("{:?}", your_category);
				self.notify.notify_one();

				self.channels_reference.insert(category.id.clone().unwrap_or("default_id".to_string()).to_string(), 
					your_category.unwrap().id.unwrap_or("default_id".to_string()).to_string());
			}

			for channel in server_data.iter().filter(|channel| channel.r#type == 0 && channel.r#type == 2) {
				tokio::time::sleep(std::time::Duration::from_secs(2)).await;

				let mut get_parent: Option<String> = None;

				if roles_params {
					for roles in channel.permission_overwrites.iter().filter(|role| role.r#type == 0) {
						println!("{:?} None error must be here", roles);
						println!("{:?}", self.roles_reference);
						permission_overwrites.push(
								ChannelPermission {
									allow: roles.allow.clone(),
									deny: roles.deny.clone(),
									id: self.roles_reference.get(&roles.id.clone()
										.to_string()).unwrap().to_string(),
									r#type: 0,
								}
							);
					}
				} 

				if channel.parent_id.is_some() {
					get_parent = Some(self.channels_reference.get(&channel.parent_id				
						.as_ref().unwrap().to_string())
						.unwrap().to_string());
				}

				let payload = DiscordChannels {
					id: None,
					name: channel.name.clone(),
					r#type: channel.r#type,
					position: channel.position,
					parent_id: get_parent,
					permission_overwrites: permission_overwrites.clone(),
				};

				let your_channel: Option<DiscordChannels> = 
					self.take_data::<DiscordChannels, DiscordChannels>(Method::POST, &format!("/guilds/{}/channels", self.your_server_id),
						Some(payload)).await;

				let mut progress_data_lock = self.progress_data.lock().await;
				*progress_data_lock = format!("{:?}", your_channel);
				self.notify.notify_one();
			}

			print!("Ended ");
		}
	}
}
