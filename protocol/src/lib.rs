#![warn(missing_docs)]
#![no_std]
//! Defines the data format for the network protocol used to communicate between
//! between the ClicKS core and ClicKS clients.

/// Definition of messages sent core -> client
pub mod message;

/// Definition of requests send client -> core
pub mod request;
