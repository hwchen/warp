//! Query Filters

use serde::de::DeserializeOwned;
use serde_qs::Config;

use ::filter::{Filter, filter_fn_one, One};
use ::reject::{self, Rejection};

/// Creates a `Filter` that decodes query parameters to the type `T`.
///
/// If cannot decode into a `T`, the request is rejected with a `400 Bad Request`.
pub fn query<T: DeserializeOwned + Send>() -> impl Filter<Extract=One<T>, Error=Rejection> + Copy {
    filter_fn_one(|route| {
        route
            .query()
            .and_then(|q| {
                lazy_static! {
                    static ref QS_NON_STRICT: Config = Config::new(5, false);
                }
                QS_NON_STRICT.deserialize_str(q)
                    .ok()
            })
            .map(Ok)
            .unwrap_or_else(|| Err(reject::bad_request()))
    })
}
