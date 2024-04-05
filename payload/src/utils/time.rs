//
//  Copyright 2024 Ram Flux, LLC.
//

use chrono::{DateTime, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}
