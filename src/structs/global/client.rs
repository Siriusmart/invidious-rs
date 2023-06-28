#[cfg(any(feature = "sync", feature = "async"))]
use crate::traits::*;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::{structs::*, INSTANCE};
#[cfg(any(feature = "sync", feature = "async"))]
use std::error::Error;

/// A blocking client struct, containing all info needed to perform a fetch.
#[cfg(feature = "sync")]
pub struct ClientSync {
    /// Method of fetching, all methods in ClientSync are blocking methods.
    pub method: MethodSync,
    /// A valid Invidious instance url.
    pub instance: String,
}

#[cfg(feature = "sync")]
impl Default for ClientSync {
    fn default() -> Self {
        Self {
            method: MethodSync::default(),
            instance: INSTANCE.to_string(),
        }
    }
}

#[cfg(feature = "sync")]
impl ClientSync {
    /// Creates new ClientSync from a given instance and method.
    pub fn with_method(instance: String, method: MethodSync) -> Self {
        Self { method, instance }
    }

    /// Takes ownership of the instance and returns a new, modifed ClientSync with changed instance.
    pub fn instance(mut self, instance: String) -> Self {
        self.set_instance(instance);
        self
    }

    /// Modifies the method of the ClientSync.
    pub fn set_method(&mut self, method: MethodSync) {
        self.method = method;
    }

    /// Takes ownership of the method and returns a new, modifed ClientSync with changed method.
    pub fn method(mut self, method: MethodSync) -> Self {
        self.set_method(method);
        self
    }
}

#[cfg(feature = "sync")]
impl ClientSyncTrait for ClientSync {
    fn new(instance: String) -> Self {
        Self {
            method: MethodSync::default(),
            instance,
        }
    }

    fn set_instance(&mut self, instance: String) {
        self.instance = instance;
    }

    fn get_instance(&self) -> &str {
        &self.instance
    }

    fn fetch(&self, url: &str) -> Result<String, Box<dyn Error>> {
        self.method.fetch(url)
    }
}

/// An async client, containing all info needed to perform a fetch.
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct ClientAsync {
    /// Method of fetching, all methods in ClientAsync are async methods.
    pub method: MethodAsync,
    /// A valid Invidious instance url.
    pub instance: String,
}

#[cfg(feature = "async")]
impl Default for ClientAsync {
    fn default() -> Self {
        Self {
            method: MethodAsync::default(),
            instance: INSTANCE.to_string(),
        }
    }
}

#[cfg(feature = "async")]
impl ClientAsync {
    /// Creates new ClientAsync from a given instance and method.
    pub fn new(instance: String, method: MethodAsync) -> Self {
        Self { method, instance }
    }

    /// Modifies the instance of the ClientAsync.
    pub fn set_instance(&mut self, instance: String) {
        self.instance = instance;
    }

    /// Takes ownership of the instance and returns a new, modifed ClientAsync with changed instance.
    pub fn instance(mut self, instance: String) -> Self {
        self.set_instance(instance);
        self
    }

    /// Modifies the method of the ClientAsync.
    pub fn set_method(&mut self, method: MethodAsync) {
        self.method = method;
    }

    /// Takes ownership of the method and returns a new, modifed ClientAsync with changed method.
    pub fn method(mut self, method: MethodAsync) -> Self {
        self.set_method(method);
        self
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl ClientAsyncTrait for ClientAsync {
    fn new(instance: String) -> Self {
        Self {
            method: MethodAsync::default(),
            instance,
        }
    }

    fn set_instance(&mut self, instance: String) {
        self.instance = instance;
    }

    fn get_instance(&self) -> &str {
        &self.instance
    }

    async fn fetch(&self, url: &str) -> Result<String, Box<dyn Error>> {
        self.method.fetch(url).await
    }
}

#[cfg(feature = "async")]
impl ClientAsyncClone for ClientAsync {}
