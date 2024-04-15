//
//  Copyright 2024 Ram Flux, LLC.
//

pub fn get_guid() -> String {
    uuid::Uuid::new_v4().to_string().to_uppercase()
}
