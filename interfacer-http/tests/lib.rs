#![feature(custom_attribute, async_await)]
#![cfg(test)]

mod http_service;

#[cfg(feature = "derive")]
mod content;
