use std::future::Future;

use crate::{AsyncMethodCustom, MethodReturn};

// dont touch this, i spent a whole hour figuring this out, its not worth wasting your life on this
pub fn into_method<F, FOutput>(f: F) -> AsyncMethodCustom
where
    F: Send + Sync + Fn(String) -> FOutput + 'static,
    FOutput: Future<Output = MethodReturn> + Send + 'static,
{
    Box::new(move |s| {
        let result = f(s);
        Box::pin(async move {
            match result.await {
                Ok(value) => Ok(value),
                Err(error) => Err(error).map_err(|err| err as Box<dyn std::error::Error>),
            }
        })
    })
}
