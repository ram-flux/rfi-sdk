//
//  Copyright 2024 Ram Flux, LLC.
//

mod api_reqwest;
pub use api_reqwest::ApiClient;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DeviceInitRes {
    osrng: String,
    pubkey: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceDel {
    pub data: String,
    pub signature: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    // use crate::fun::get_guid;

    async fn init_device() -> anyhow::Result<Option<DeviceInitRes>> {
        let api_client = ApiClient::new();
        let url = "http://127.0.0.1:8117/v1/device";
        let body = json!({
            "account": "038f3693e749f470accf6519a9af48667474c5da4c3b2c4d600102a423794b83eb",
            "signature": "304402204889e3ebd37649d9f707848da0677d0fb09057fbecbfed74e71b20bba953a785022009f0d65d3f53083793fff85c33ce7fe53ec74a66f7191e13bbccff9ee91d0069",
            "uuid": "382C2668-6E36-4C0E-97E3-BF11DB0424E9",
        })
        .to_string();
        println!("body: {}", body);
        let headers = Some(vec![("Content-Type", "application/json")]);
        let response = api_client
            .send(reqwest::Method::POST, url, Some(body), headers)
            .await
            .expect("Failed to send request");
        println!("response: {:?}", response);

        let parsed_response = ApiClient::from_json::<DeviceInitRes>(&response).await;
        parsed_response
    }

    async fn binding_device(res: DeviceInitRes) -> anyhow::Result<(String, String, String)> {
        println!("res: {:?}", res);

        let api_client = ApiClient::new();
        let url = "http://127.0.0.1:8117/v1/device/binding";

        let acc = "038f3693e749f470accf6519a9af48667474c5da4c3b2c4d600102a423794b83eb";
        let device_id = "382C2668-6E36-4C0E-97E3-BF11DB0424E9";

        let (device_pri, device_pub) = wallet::Device::generate_device_key(
            device_id.as_bytes(),
            acc.as_bytes(),
            res.osrng.clone(),
        )
        .unwrap();

        let device_pri_str = hex::encode(device_pri.as_bytes());
        let device_pub_str = hex::encode(device_pub.as_bytes());

        let body = json!({
            "device_pubkey": device_pub_str.clone(),
        });

        let body_str = wallet::Encrypt::new(
            device_pri_str.clone(),
            res.pubkey.clone(),
            "unique nonce".to_string(),
            body.to_string(),
        )
        .encrypt()
        .unwrap();
        let data = format!("{}.{}.{}", body_str, res.osrng, res.pubkey.clone());
        let ser_pubkey = res.pubkey;

        let headers = Some(vec![("Content-Type", "application/json")]);
        let response = api_client
            .send(reqwest::Method::POST, url, Some(data), headers)
            .await
            .expect("Failed to send request");
        println!("response: {:?}", response);
        Ok((device_pri_str, device_pub_str, ser_pubkey))
    }

    async fn del_device(device_pri: String, device_pub: String, ser_pubkey: String) {
        // println!("device_pri: {:?}", device_pri);
        // println!("device_pub: {:?}", device_pub);
        // println!("ser_pubkey: {:?}", ser_pubkey);

        let api_client = ApiClient::new();
        let url = "http://127.0.0.1:8117/v1/device";

        // let acc = "038f3693e749f470accf6519a9af48667474c5da4c3b2c4d600102a423794b83eb";
        let device_id = "382C2668-6E36-4C0E-97E3-BF11DB0424E9";

        let payload = wallet::Encrypt::new(
            device_pri.clone(),
            ser_pubkey.clone(),
            "unique nonce".to_string(),
            device_id.to_string(),
        )
        .encrypt()
        .unwrap();
        let data = format!("{}.{}", payload.clone(), device_pub);

        let sig = wallet::Signature::new(device_pri, ser_pubkey)
            .sign(&payload)
            .unwrap();

        // println!("sig: {:?}", sig);

        let body = json!({
            "data": data,
            "signature": sig,
        });

        let headers = Some(vec![("Content-Type", "application/json")]);
        let response = api_client
            .send(
                reqwest::Method::DELETE,
                url,
                Some(body.to_string()),
                headers,
            )
            .await
            .expect("Failed to send request");
        println!("response: {:?}", response);
    }

    /**
     * Test device in server pubkey
     */
    async fn update_device(device_pri: String, device_pub: String, ser_pubkey: String) {
        let api_client = ApiClient::new();
        let url = "http://127.0.0.1:8117/v1/device";

        // let acc = "038f3693e749f470accf6519a9af48667474c5da4c3b2c4d600102a423794b83eb";
        let device_id = "382C2668-6E36-4C0E-97E3-BF11DB0424E9";

        let payload = wallet::Encrypt::new(
            device_pri.clone(),
            ser_pubkey.clone(),
            "unique nonce".to_string(),
            device_id.to_string(),
        )
        .encrypt()
        .unwrap();
        let data = format!("{}.{}", payload.clone(), device_pub);

        let sig = wallet::Signature::new(device_pri, ser_pubkey)
            .sign(&payload)
            .unwrap();

        let body = json!({
            "data": data,
            "signature": sig,
        });

        let headers = Some(vec![("Content-Type", "application/json")]);
        let response = api_client
            .send(reqwest::Method::PUT, url, Some(body.to_string()), headers)
            .await
            .expect("Failed to send request");
        println!("response: {:?}", response);
    }

    //cargo test --package http --lib -- util::tests::test_device --exact --nocapture
    #[tokio::test]
    async fn test_device() {
        let parsed_response = init_device().await.unwrap();
        if let Some(data) = parsed_response {
            // println!("Received data: {:?}", data);
            let (pri, pubk, ser_pubk) = binding_device(data).await.unwrap();
            update_device(pri, pubk, ser_pubk).await;
            // del_device(pri, pubk, ser_pubk).await;
        } else {
            println!("No data in the response.");
        }
    }
}
