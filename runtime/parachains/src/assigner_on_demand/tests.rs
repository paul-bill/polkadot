// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use crate::{
	assigner_on_demand::Error,
	initializer::SessionChangeNotification,
	mock::{
		new_test_ext, MockGenesisConfig, OnDemandAssigner, Paras, ParasShared, RuntimeOrigin,
		Scheduler, System, Test,
	},
	paras::{ParaGenesisArgs, ParaKind},
};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use primitives::{
	v5::{Assignment, ValidationCode},
	Balance, BlockNumber, SessionIndex,
};
use sp_std::collections::btree_map::BTreeMap;

fn default_genesis_config() -> MockGenesisConfig {
	MockGenesisConfig {
		configuration: crate::configuration::GenesisConfig {
			config: crate::configuration::HostConfiguration { ..Default::default() },
		},
		..Default::default()
	}
}

fn schedule_blank_para(id: ParaId, parakind: ParaKind) {
	let validation_code: ValidationCode = vec![1, 2, 3].into();
	assert_ok!(Paras::schedule_para_initialize(
		id,
		ParaGenesisArgs {
			genesis_head: Vec::new().into(),
			validation_code: validation_code.clone(),
			para_kind: parakind,
		}
	));

	assert_ok!(Paras::add_trusted_validation_code(RuntimeOrigin::root(), validation_code));
}

fn run_to_block(
	to: BlockNumber,
	new_session: impl Fn(BlockNumber) -> Option<SessionChangeNotification<BlockNumber>>,
) {
	while System::block_number() < to {
		let b = System::block_number();

		Scheduler::initializer_finalize();
		Paras::initializer_finalize(b);

		if let Some(notification) = new_session(b + 1) {
			let mut notification_with_session_index = notification;
			// We will make every session change trigger an action queue. Normally this may require 2 or more session changes.
			if notification_with_session_index.session_index == SessionIndex::default() {
				notification_with_session_index.session_index = ParasShared::scheduled_session();
			}
			Paras::initializer_on_new_session(&notification_with_session_index);
			Scheduler::initializer_on_new_session(&notification_with_session_index);
		}

		System::on_finalize(b);

		System::on_initialize(b + 1);
		System::set_block_number(b + 1);

		Paras::initializer_initialize(b + 1);
		Scheduler::initializer_initialize(b + 1);

		// In the real runtime this is expected to be called by the `InclusionInherent` pallet.
		Scheduler::update_claimqueue(BTreeMap::new(), b + 1);
	}
}

#[derive(Debug)]
pub(super) struct GenesisConfigBuilder {
	parathread_cores: u32,
	on_demand_base_fee: Balance,
	on_demand_fee_variability: Perbill,
	on_demand_max_queue_size: u32,
	on_demand_target_queue_utilization: Perbill,
}

impl Default for GenesisConfigBuilder {
	fn default() -> Self {
		Self {
			parathread_cores: 10,
			on_demand_base_fee: 10_000,
			on_demand_fee_variability: Perbill::from_percent(1),
			on_demand_max_queue_size: 100,
			on_demand_target_queue_utilization: Perbill::from_percent(25),
		}
	}
}

impl GenesisConfigBuilder {
	pub(super) fn build(self) -> crate::mock::MockGenesisConfig {
		let mut genesis = default_genesis_config();
		let config = &mut genesis.configuration.config;
		config.on_demand_cores = self.parathread_cores;
		config.on_demand_base_fee = self.on_demand_base_fee;
		config.on_demand_fee_variability = self.on_demand_fee_variability;
		config.on_demand_queue_max_size = self.on_demand_max_queue_size;
		config.on_demand_target_queue_utilization = self.on_demand_target_queue_utilization;
		genesis
	}
}

#[test]
fn spot_traffic_capacity_zero_returns_none() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		0u32,
		u32::MAX,
		Perbill::from_percent(100),
		Perbill::from_percent(1),
	) {
		Ok(_) => panic!("Error"),
		Err(e) => assert_eq!(e, SpotTrafficCalculationErr::QueueCapacityIsZero),
	};
}

#[test]
fn spot_traffic_queue_size_larger_than_capacity_returns_none() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		1u32,
		2u32,
		Perbill::from_percent(100),
		Perbill::from_percent(1),
	) {
		Ok(_) => panic!("Error"),
		Err(e) => assert_eq!(e, SpotTrafficCalculationErr::QueueSizeLargerThanCapacity),
	}
}

#[test]
fn spot_traffic_calculation_identity() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from_u32(1),
		1000,
		100,
		Perbill::from_percent(10),
		Perbill::from_percent(3),
	) {
		Ok(res) => {
			assert_eq!(res, FixedU128::from_u32(1))
		},
		_ => (),
	}
}

#[test]
fn spot_traffic_calculation_u32_max() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from_u32(1),
		u32::MAX,
		u32::MAX,
		Perbill::from_percent(100),
		Perbill::from_percent(3),
	) {
		Ok(res) => {
			assert_eq!(res, FixedU128::from_u32(1))
		},
		_ => panic!("Error"),
	};
}

#[test]
fn spot_traffic_calculation_u32_traffic_max() {
	match OnDemandAssigner::calculate_spot_traffic(
		FixedU128::from(u128::MAX),
		u32::MAX,
		u32::MAX,
		Perbill::from_percent(1),
		Perbill::from_percent(1),
	) {
		Ok(res) => assert_eq!(res, FixedU128::from(u128::MAX)),
		_ => panic!("Error"),
	};
}

#[test]
fn sustained_target_increases_spot_traffic() {
	let mut traffic = FixedU128::from_u32(1u32);
	for _ in 0..50 {
		traffic = OnDemandAssigner::calculate_spot_traffic(
			traffic,
			100,
			12,
			Perbill::from_percent(10),
			Perbill::from_percent(100),
		)
		.unwrap()
	}
	assert_eq!(traffic, FixedU128::from_inner(2_718_103_312_071_174_015u128))
}

#[test]
fn spot_traffic_can_decrease() {
	let traffic = FixedU128::from_u32(100u32);
	match OnDemandAssigner::calculate_spot_traffic(
		traffic,
		100u32,
		0u32,
		Perbill::from_percent(100),
		Perbill::from_percent(100),
	) {
		Ok(new_traffic) =>
			assert_eq!(new_traffic, FixedU128::from_inner(50_000_000_000_000_000_000u128)),
		_ => panic!("Error"),
	}
}

#[test]
fn spot_traffic_decreases_over_time() {
	let mut traffic = FixedU128::from_u32(100u32);
	for _ in 0..5 {
		traffic = OnDemandAssigner::calculate_spot_traffic(
			traffic,
			100u32,
			0u32,
			Perbill::from_percent(100),
			Perbill::from_percent(100),
		)
		.unwrap();
		println!("{traffic}");
	}
	assert_eq!(traffic, FixedU128::from_inner(3_125_000_000_000_000_000u128))
}

#[test]
fn place_order_works() {
	let alice = 1u64;
	let amt = 10_000_000u128;
	let para_a = ParaId::from(111);

	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		// Initialize the parathread and wait for it to be ready.
		schedule_blank_para(para_a, ParaKind::Parathread);

		assert!(!Paras::is_parathread(para_a));

		run_to_block(100, |n| if n == 100 { Some(Default::default()) } else { None });

		assert!(Paras::is_parathread(para_a));

		// should the account be reaped
		let reap_account = true;

		// Does not work unsigned
		assert_noop!(
			OnDemandAssigner::place_order(RuntimeOrigin::none(), amt, para_a, reap_account),
			BadOrigin
		);

		// Does not work with max_amount lower than fee
		let low_max_amt = 1u128;
		assert_noop!(
			OnDemandAssigner::place_order(
				RuntimeOrigin::signed(alice),
				low_max_amt,
				para_a,
				reap_account
			),
			Error::<Test>::SpotPriceHigherThanMaxAmount,
		);
	});
}

#[test]
fn spotqueue_push_directions() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let para_b = ParaId::from(222);
		let para_c = ParaId::from(333);

		schedule_blank_para(para_a, ParaKind::Parathread);
		schedule_blank_para(para_b, ParaKind::Parathread);
		schedule_blank_para(para_c, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let assignment_a = Assignment { para_id: para_a };
		let assignment_b = Assignment { para_id: para_b };
		let assignment_c = Assignment { para_id: para_c };

		assert_ok!(OnDemandAssigner::add_parathread_assignment(
			assignment_a.clone(),
			QueuePushDirection::Front
		));
		assert_ok!(OnDemandAssigner::add_parathread_assignment(
			assignment_b.clone(),
			QueuePushDirection::Front
		));

		assert_ok!(OnDemandAssigner::add_parathread_assignment(
			assignment_c.clone(),
			QueuePushDirection::Back
		));

		assert_eq!(OnDemandAssigner::queue_size(), 3);
		assert_eq!(
			OnDemandAssigner::get_queue(),
			VecDeque::from(vec![assignment_b, assignment_a, assignment_c])
		)
	});
}

#[test]
fn affinity_changes_work() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		schedule_blank_para(para_a, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let assignment_a = Assignment { para_id: para_a };
		// There should be no affinity before starting.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());

		// Add enough assignments to the order queue.
		for _ in 0..10 {
			OnDemandAssigner::add_parathread_assignment(
				assignment_a.clone(),
				QueuePushDirection::Front,
			)
			.expect("Invalid paraid or queue full");
		}

		// There should be no affinity before the scheduler pops.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());

		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), None);

		// Affinity count is 1 after popping.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 1);

		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_a.clone()));

		// Affinity count is 1 after popping with a previous para.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 1);
		assert_eq!(OnDemandAssigner::queue_size(), 8);

		for _ in 0..3 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), None);
		}

		// Affinity count is 4 after popping 3 times without a previous para.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 4);
		assert_eq!(OnDemandAssigner::queue_size(), 5);

		for _ in 0..5 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_a.clone()));
		}

		// Affinity count should still be 4 but queue should be empty.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 4);
		assert_eq!(OnDemandAssigner::queue_size(), 0);

		// Pop 4 times and get to exactly 0 (None) affinity.
		for _ in 0..4 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_a.clone()));
		}
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());

		// Decreasing affinity beyond 0 should still be None.
		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_a.clone()));
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());
	});
}

#[test]
fn affinity_prohibits_parallel_scheduling() {
	new_test_ext(GenesisConfigBuilder::default().build()).execute_with(|| {
		let para_a = ParaId::from(111);
		let para_b = ParaId::from(222);

		schedule_blank_para(para_a, ParaKind::Parathread);
		schedule_blank_para(para_b, ParaKind::Parathread);

		run_to_block(11, |n| if n == 11 { Some(Default::default()) } else { None });

		let assignment_a = Assignment { para_id: para_a };
		let assignment_b = Assignment { para_id: para_b };

		// There should be no affinity before starting.
		assert!(OnDemandAssigner::get_affinity_map(para_a).is_none());
		assert!(OnDemandAssigner::get_affinity_map(para_b).is_none());

		// Add 2 assignments for para_a for every para_b.
		OnDemandAssigner::add_parathread_assignment(assignment_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_parathread_assignment(assignment_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_parathread_assignment(assignment_b.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		assert_eq!(OnDemandAssigner::queue_size(), 3);

		// Approximate having 1 core.
		for _ in 0..3 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), None);
		}

		// Affinity on one core is meaningless.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 2);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_b).unwrap().count, 1);
		assert_eq!(
			OnDemandAssigner::get_affinity_map(para_a).unwrap().core_idx,
			OnDemandAssigner::get_affinity_map(para_b).unwrap().core_idx
		);

		// Clear affinity
		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_a.clone()));
		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_a.clone()));
		OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), Some(para_b.clone()));

		// Add 2 assignments for para_a for every para_b.
		OnDemandAssigner::add_parathread_assignment(assignment_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_parathread_assignment(assignment_a.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		OnDemandAssigner::add_parathread_assignment(assignment_b.clone(), QueuePushDirection::Back)
			.expect("Invalid paraid or queue full");

		// Approximate having 2 cores.
		for _ in 0..3 {
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(0), None);
			OnDemandAssigner::pop_assignment_for_core(CoreIndex(1), None);
		}

		// Affinity should be the same as before, but on different cores.
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().count, 2);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_b).unwrap().count, 1);
		assert_eq!(OnDemandAssigner::get_affinity_map(para_a).unwrap().core_idx, CoreIndex(0));
		assert_eq!(OnDemandAssigner::get_affinity_map(para_b).unwrap().core_idx, CoreIndex(1));
	});
}