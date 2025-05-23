//! # Stock Market Indicators
//!
//! This module provides indicators specialized for stock/equity markets.
//!
//! ## Available Indicator Groups
//!
//! - [`price_action`](price_action/index.html): Indicators based on price action specific to stocks
//! - [`fundamental`](fundamental/index.html): Indicators incorporating fundamental data with technical indicators

pub mod fundamental;
pub mod price_action;

// Re-export common types and functions for convenient access
pub use fundamental::FundamentalIndicators;
pub use price_action::StockPricePatterns;
