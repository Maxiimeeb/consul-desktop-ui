pub mod client;
mod kv;

use serde_json::{Map, Value};
use std::collections::HashMap;

/// Insert a value into a JSON object at a given path.
/// If the path does not exist, it will be created.
/// If the path already exists, the value will be overwritten.
/// Doesn't support arrays in the path.
///
/// # Arguments
/// * `json` - The JSON object to insert the value into.
/// * `path` - The path to insert the value at.
/// * `value` - The value to insert.
///
/// # Example
/// ```r
/// use serde_json::{Map, Value};
/// let mut json: Map<String, Value> = Map::new();
///
/// insert_value(&mut json, vec!["key", "path"], Value::String("value".to_string()));
/// assert_eq!(json.get("key").unwrap().get("path").unwrap(), &Value::String("value".to_string()));
/// ```
fn insert_value(json: &mut Map<String, Value>, path: Vec<&str>, value: Value) {
    let mut current = json;
    for i in 0..path.len() {
        if i == path.len() - 1 {
            current.insert(path[i].to_string(), value);
            break;
        } else {
            if !current.contains_key(path[i]) {
                current.insert(path[i].to_string(), Value::Object(Map::new()));
            }
            current = current.get_mut(path[i]).unwrap().as_object_mut().unwrap();
        }
    }
}

/// Generate a HashMap of key-value pairs from a JSON object.
/// Will perform a recursive search.
///
/// # Returns
/// A HashMap of key-value pairs
///
/// # Example
/// ```r
/// use serde_json::{json, Map, Value};
///
/// let mut json: json!({
///    "key1": "value1",
///   "key2": {
///    "key3": "value3"
///  }
/// });
///
/// let key_values = generate_key_values(&json);
/// // key1 -> value1
/// // key2/key3 -> value3
/// ```
///
fn generate_key_values(json: &Map<String, Value>) -> Result<HashMap<String, String>, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for (key, value) in json.iter() {
        if value.is_object() {
            let nested = generate_key_values(value.as_object().unwrap())?;

            for (nested_key, nested_value) in nested.iter() {
                result.insert(format!("{}/{}", key, nested_key), nested_value.to_string());
            }
        } else {
            let as_str = value.as_str();
            match as_str {
                Some(v) => result.insert(key.clone(), v.to_string()),
                None => return Err(format!("Value for key {} is not a string", key)),
            };
        }
    }

    Ok(result)
}

/// Update a Consul key-value store with new values.
/// This will:
/// - Remove keys that are not present in the new values.
/// - Add keys that are not present in the old values.
/// - Update keys that have changed values.
///
/// # Example
/// ```r
/// mod consul;
///
/// use std::ops::Deref;
/// use serde_json::{to_string, json, Value};
/// use consul::client::{ConsulClient, Schema, Host, Port};
///
/// #[tokio::main]
/// async fn main() {
///     // Create a client
///     let client = ConsulClient {
///         host: Host::new("localhost".to_string()),
///         port: Port::new(8500).unwrap(),
///         scheme: Schema::Http,
///     };
///     let values = consul::ConsulValues::new_from_client(&client).await;
///     let json = values.to_json();
///
///     let mut x = json!({
///         "key1": "value1",
///         "key2": {
///             "key3": "value3",
///         }
///     });
///     let y = x.as_object_mut().unwrap().clone();
///
///     let values2 = consul::ConsulValues::new_from_json(y);
///
///     consul::update_consul(&client, &values, &values2).await;
/// }
/// ```

pub async fn update_consul(
    client: &client::ConsulClient,
    old_values: &ConsulValues,
    new_values: &ConsulValues,
) -> Result<(), Box<dyn std::error::Error>> {
    let to_remove_keys: Vec<String> = old_values
        .key_values
        .keys()
        .filter(|key| !new_values.key_values.contains_key(*key))
        .map(|key| key.clone())
        .collect();
    let keys_to_add: Vec<String> = new_values
        .key_values
        .keys()
        .filter(|key| !old_values.key_values.contains_key(*key))
        .map(|key| key.clone())
        .collect();
    let keys_to_update: Vec<String> = new_values
        .key_values
        .keys()
        .filter(|key| old_values.key_values.contains_key(*key))
        .filter(|key| {
            old_values.key_values.get(*key).unwrap() != new_values.key_values.get(*key).unwrap()
        })
        .map(|key| key.clone())
        .collect();

    for key in to_remove_keys {
        kv::delete(client, &key).await?;
    }

    for key in keys_to_add.iter().chain(&keys_to_update) {
        let value = new_values.key_values.get(key).unwrap();
        kv::set(client, &key, &value).await?;
    }

    Ok(())
}

/// Check if a JSON object has no arrays in it. Will perform a recursive search.
fn has_no_array(json: &Map<String, Value>) -> bool {
    for (_, value) in json.iter() {
        if value.is_array() {
            return false;
        }
        if value.is_object() {
            if !has_no_array(value.as_object().unwrap()) {
                return false;
            }
        }
    }

    true
}

pub struct ConsulValues {
    internal_values: Map<String, Value>,
    key_values: HashMap<String, String>,
}

impl ConsulValues {
    pub async fn new_from_client(
        client: &client::ConsulClient,
    ) -> Result<ConsulValues, Box<dyn std::error::Error>> {
        let keys = kv::list_all_keys(client).await?;
        let mut key_values: HashMap<String, String> = HashMap::new();

        let mut internal_values: Map<String, Value> = Map::new();

        for key in keys {
            let value = kv::read(client, &key).await?;

            if value.is_none() {
                continue;
            }
            let path = key.split("/").collect::<Vec<&str>>();
            let value = value.unwrap().try_into().unwrap();
            insert_value(
                &mut internal_values,
                path,
                Value::String(String::from(&value)),
            );
            key_values.insert(key, value);
        }

        Ok(ConsulValues {
            internal_values,
            key_values,
        })
    }

    pub fn new_from_json(values: Map<String, Value>) -> Result<ConsulValues, String> {
        if !has_no_array(&values) {
            panic!("JSON object cannot contain arrays.");
        }

        let key_values = generate_key_values(&values)?;

        Ok(ConsulValues {
            internal_values: values,
            key_values,
        })
    }

    pub fn to_json(&self) -> Map<String, Value> {
        self.internal_values.clone()
    }
}
