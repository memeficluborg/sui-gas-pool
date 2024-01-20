// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mysten_metrics::histogram::Histogram;
use prometheus::{
    register_int_counter_vec_with_registry, register_int_counter_with_registry,
    register_int_gauge_vec_with_registry, register_int_gauge_with_registry, IntCounter,
    IntCounterVec, IntGauge, IntGaugeVec, Registry,
};
use std::sync::Arc;
use tracing::error;

pub struct GasPoolMetrics {
    // === RPC Server Metrics ===
    // RPC metrics for the reserve_gas endpoint
    pub num_total_reserve_gas_requests: IntCounter,
    pub num_authorized_reserve_gas_requests: IntCounter,
    pub num_successful_reserve_gas_requests: IntCounter,

    // Statistics about the gas reservation request
    pub target_gas_budget_per_request: Histogram,
    pub reserve_duration_per_request: Histogram,

    // Statistics about the gas reservation response
    pub reserved_gas_coin_count_per_request: Histogram,

    // RPC metrics for the execute_tx endpoint
    pub num_total_execute_tx_requests: IntCounter,
    pub num_authorized_execute_tx_requests: IntCounter,
    pub num_successful_execute_tx_requests: IntCounter,

    // === Gas Station Metrics ===
    pub num_successful_storage_pool_reservation: IntCounter,
    pub num_failed_storage_pool_reservation: IntCounter,

    pub cur_num_alive_reservations: IntGauge,
    pub cur_num_reserved_gas_coins: IntGauge,

    pub num_expired_reservations: IntCounter,
    pub num_expired_gas_coins: IntCounter,

    pub num_released_reservations: IntCounter,
    pub num_released_gas_coins: IntCounter,
    pub reserved_duration_upon_release: Histogram,
    pub num_gas_coins_smashed: IntCounter,

    pub num_gas_pool_invariant_violations: IntCounter,
}

impl GasPoolMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        Arc::new(Self {
            num_total_reserve_gas_requests: register_int_counter_with_registry!(
                "num_reserve_gas_requests",
                "Total number of reserve_gas RPC requests received",
                registry,
            )
            .unwrap(),
            num_authorized_reserve_gas_requests: register_int_counter_with_registry!(
                "num_authorized_reserve_gas_requests",
                "Total number of reserve_gas RPC requests that provided the correct auth token",
                registry,
            )
            .unwrap(),
            num_successful_reserve_gas_requests: register_int_counter_with_registry!(
                "num_successful_reserve_gas_requests",
                "Total number of reserve_gas RPC requests that were successful",
                registry,
            )
            .unwrap(),
            target_gas_budget_per_request: Histogram::new_in_registry(
                "target_gas_budget_per_request",
                "Target gas budget value in the reserve_gas RPC request",
                registry,
            ),
            reserve_duration_per_request: Histogram::new_in_registry(
                "reserve_duration_per_request",
                "Reserve duration value in the reserve_gas RPC request",
                registry,
            ),
            reserved_gas_coin_count_per_request: Histogram::new_in_registry(
                "gas_coin_count_per_request",
                "Number of gas coins reserved in the reserve_gas RPC response",
                registry,
            ),
            num_total_execute_tx_requests: register_int_counter_with_registry!(
                "num_execute_tx_requests",
                "Total number of execute_tx RPC requests received",
                registry,
            )
            .unwrap(),
            num_authorized_execute_tx_requests: register_int_counter_with_registry!(
                "num_authorized_execute_tx_requests",
                "Total number of execute_tx RPC requests that provided the correct auth token",
                registry,
            )
            .unwrap(),
            num_successful_execute_tx_requests: register_int_counter_with_registry!(
                "num_successful_execute_tx_requests",
                "Total number of execute_tx RPC requests that were successful",
                registry,
            )
            .unwrap(),
            num_successful_storage_pool_reservation: register_int_counter_with_registry!(
                "num_successful_storage_pool_reservation",
                "Total number of successful storage pool reservation requests",
                registry,
            )
            .unwrap(),
            num_failed_storage_pool_reservation: register_int_counter_with_registry!(
                "num_failed_storage_pool_reservation",
                "Total number of failed storage pool reservation requests",
                registry,
            )
            .unwrap(),
            cur_num_alive_reservations: register_int_gauge_with_registry!(
                "num_alive_reservations",
                "Number of alive reservations that have not expired yet",
                registry,
            )
            .unwrap(),
            cur_num_reserved_gas_coins: register_int_gauge_with_registry!(
                "num_reserved_gas_coins",
                "Number of gas coins currently reserved",
                registry,
            )
            .unwrap(),
            num_expired_reservations: register_int_counter_with_registry!(
                "num_expired_reservations",
                "Total number of expired reservations",
                registry,
            )
            .unwrap(),
            num_expired_gas_coins: register_int_counter_with_registry!(
                "num_expired_gas_coins",
                "Total number of expired gas coins",
                registry,
            )
            .unwrap(),
            num_released_reservations: register_int_counter_with_registry!(
                "num_released_reservations",
                "Total number of released reservations from proactive transaction execution",
                registry,
            )
            .unwrap(),
            num_released_gas_coins: register_int_counter_with_registry!(
                "num_released_gas_coins",
                "Total number of released gas coins from proactive transaction execution",
                registry,
            )
            .unwrap(),
            reserved_duration_upon_release: Histogram::new_in_registry(
                "reserved_duration_upon_release",
                "Reservation duration from when the coins were reserved to when they were released through transaction execution",
                registry,
            ),
            num_gas_coins_smashed: register_int_counter_with_registry!(
                "num_gas_coins_smashed",
                "Total number of gas coins smashed during transaction execution",
                registry,
            )
            .unwrap(),
            num_gas_pool_invariant_violations: register_int_counter_with_registry!(
                "num_gas_pool_invariant_violations",
                "Total number of invariant violations in the gas pool. This should really never trigger",
                registry,
            )
            .unwrap(),
        })
    }

    pub fn new_for_testing() -> Arc<Self> {
        Self::new(&Registry::new())
    }

    pub fn invariant_violation<T: Into<String>>(&self, msg: T) {
        if cfg!(debug_assertions) {
            panic!("Invariant violation: {}", msg.into());
        } else {
            error!("Invariant violation: {}", msg.into());
        }
        self.num_gas_pool_invariant_violations.inc();
    }
}

pub struct StoragePoolMetrics {
    // TODO: Add more metrics on storage request, newly added APIs and etc

    // === Storage Pool Metrics
    pub num_total_storage_reserve_gas_coins_requests: IntCounterVec,
    pub num_successful_storage_reserve_gas_coins_requests: IntCounterVec,
    pub num_total_storage_update_gas_coins_requests: IntCounterVec,
    pub num_successful_storage_update_gas_coins_requests: IntCounterVec,

    pub cur_num_available_gas_coins: IntGaugeVec,
    pub cur_total_available_gas_balance: IntGaugeVec,
    pub cur_num_reserved_gas_coins: IntGaugeVec,

    pub num_storage_invariant_violations: IntCounter,
}

impl StoragePoolMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        Arc::new(Self {
            cur_num_available_gas_coins: register_int_gauge_vec_with_registry!(
                "cur_num_available_gas_coins",
                "Current number of available gas coins",
                &["address"],
                registry,
            )
            .unwrap(),
            cur_total_available_gas_balance: register_int_gauge_vec_with_registry!(
                "cur_total_available_gas_balance",
                "Current total available gas coin balance",
                &["address"],
                registry,
            )
            .unwrap(),
            cur_num_reserved_gas_coins: register_int_gauge_vec_with_registry!(
                "cur_num_reserved_gas_coins",
                "Current number of reserved gas coins",
                &["address"],
                registry,
            )
            .unwrap(),
            num_total_storage_reserve_gas_coins_requests: register_int_counter_vec_with_registry!(
                "num_total_storage_reserve_gas_coins_requests",
                "Total number of storage pool reserve_gas_coins requests received",
                &["address"],
                registry,
            )
            .unwrap(),
            num_successful_storage_reserve_gas_coins_requests:
                register_int_counter_vec_with_registry!(
                    "num_successful_storage_reserve_gas_coins_requests",
                    "Total number of storage pool reserve_gas_coins requests that were successful",
                    &["address"],
                    registry,
                )
                .unwrap(),
            num_total_storage_update_gas_coins_requests: register_int_counter_vec_with_registry!(
                "num_total_storage_update_gas_coins_requests",
                "Total number of storage pool update_gas_coins requests received",
                &["address"],
                registry,
            )
            .unwrap(),
            num_successful_storage_update_gas_coins_requests:
                register_int_counter_vec_with_registry!(
                    "num_successful_storage_update_gas_coins_requests",
                    "Total number of storage pool update_gas_coins requests that were successful",
                    &["address"],
                    registry,
                )
                .unwrap(),
            num_storage_invariant_violations: register_int_counter_with_registry!(
                "num_storage_invariant_violations",
                "Total number of invariant violations in the storage. This should really never trigger",
                registry,
            )
            .unwrap(),
        })
    }

    pub fn invariant_violation<T: Into<String>>(&self, msg: T) {
        if cfg!(debug_assertions) {
            panic!("Invariant violation: {}", msg.into());
        } else {
            error!("Invariant violation: {}", msg.into());
        }
        self.num_storage_invariant_violations.inc();
    }

    pub fn new_for_testing() -> Arc<Self> {
        Self::new(&Registry::new())
    }
}