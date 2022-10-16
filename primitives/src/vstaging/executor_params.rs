// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! Abstract execution environment parameter set.
//!
//! Parameters are encoded as (u8, u64) tuples, where u8 is a tag defining a type of
//! parameter, and u64 is a parameter value. Parameter set and value structure are opaque
//! and depend on execution environment itself. Decoding to a usable semantics structure is
//! done in `polkadot-node-core-pvf`.

use parity_scale_codec::{Decode, Encode};
use parity_util_mem::MallocSizeOf;
use scale_info::TypeInfo;
use sp_std::{ops::Deref, vec::Vec};

// // FIXME: Version structure is to be determined
// /// Tag for version of execution environment parameter set.
// pub const PAR_VERSION: u8 = 0;
// /// Tag for extra heap page count
// pub const PAR_EXTRA_HEAP_PAGES: u8 = 1;
// /// Tag for max. memory size
// pub const PAR_MAX_MEMORY_SIZE: u8 = 2;
// /// Tag for stack limits. 32 LSB bits define logical limit, and 32 MSB bits define native stack limit
// pub const PAR_STACK_LIMIT: u8 = 3;
// /// Tag for bit-packed fields
// pub const PAR_BITS: u8 = 4;

// /// Total number of tags (for vector pre-allocation)
// pub const PAR_LEN: usize = 5;

// /// Bitflag for NaNs canonicalization boolean
// pub const BIT_CANONICAL_NANS: u8 = 0;
// /// Bitflag fof parallel compilation boolean
// pub const BIT_PARALLEL_COMPILATION: u8 = 1;

// /// Insantiation strategy bit-packed constants:
// /// Pooling copy-on-write
// pub const INST_POOLING_COW: u8 = 0b001;
// /// Recreate copy-on-write
// pub const INST_RECREATE_COW: u8 = 0b010;
// /// Pooling
// pub const INST_POOLING: u8 = 0b011;
// /// Recreate
// pub const INST_RECREATE: u8 = 0b100;
// /// Legacy instantiation strategy
// pub const INST_LEGACY: u8 = 0b101;

/// Execution environment type
#[derive(Clone, Copy, Debug, Encode, Decode, PartialEq, Eq, MallocSizeOf, TypeInfo)]
pub enum ExecutionEnvironment {
	/// Generic Wasmtime executor
	WasmtimeGeneric = 0,
}

/// Executor instantiation strategy
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, MallocSizeOf, TypeInfo)]
pub enum ExecInstantiationStrategy {
	/// Pooling copy-on-write
	PoolingCoW,
	/// Recreate instance copy-on-write
	RecreateCoW,
	/// Pooling
	Pooling,
	/// Recreate instance
	Recreate,
	/// Legacy instantiation strategy
	Legacy,
}

/// A single executor parameter
#[non_exhaustive]
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, MallocSizeOf, TypeInfo)]
pub enum ExecutorParam {
	/// General parameters:
	/// Execution environment type
	Environment(ExecutionEnvironment),
	/// Version of the set of execution parameters, unique in the scope of execution environment type
	Version(u32),

	/// Parameters setting the executuion environment semantics:
	/// Number of extra heap pages
	ExtraHeapPages(u64),
	/// Max. memory size
	MaxMemorySize(u32),
	/// Wasm logical stack size limit, in units
	StackLogicalMax(u32),
	/// Executor machine stack size limit, in bytes
	StackNativeMax(u32),
	/// Executor instantiation strategy
	InstantiationStrategy(ExecInstantiationStrategy),
	/// `true` if compiler should perform NaNs canonicalization
	CanonicalizeNaNs(bool),
	/// `true` if parallel compilation is allowed, single thread is used otherwise
	ParallelCompilation(bool),
}

/// Deterministically serialized execution environment semantics
///
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq, MallocSizeOf, TypeInfo)]
pub struct ExecutorParams(Vec<ExecutorParam>);
// pub struct ExecutorParams(Vec<(u8, u64)>);

impl ExecutorParams {
	// /// Returns version of execution environment parameter set, if present. Otherwise, returns 0.
	// pub fn version(&self) -> u64 {
	// 	if self.0.len() == 0 {
	// 		0
	// 	} else {
	// 		if self.0[0].0 == PAR_VERSION {
	// 			self.0[0].1
	// 		} else {
	// 			0
	// 		}
	// 	}
	// }

	/// Returns globally unique version of execution environment parameter set, if present. Otherwise, returns 0.
	pub fn version(&self) -> u64 {
		if self.0.len() < 2 {
			return 0
		}
		let env = if let ExecutorParam::Environment(env) = self.0[0] { env } else { return 0 };
		let ver = if let ExecutorParam::Version(ver) = self.0[1] { ver } else { return 0 };
		// let env = match ExecutorParam::decode(&mut &self.0[0].clone()[..]) {
		// 	Ok(ExecutorParam::Environment(v)) => v,
		// 	_ => return 0,
		// };
		// let ver = match ExecutorParam::decode(&mut &self.0[1].clone()[..]) {
		// 	Ok(ExecutorParam::Version(v)) => v,
		// 	_ => return 0,
		// };
		(env as u64) * 2 ^ 32u64 + ver as u64
	}

	/// Returns execution environment type identifier
	pub fn environment(&self) -> ExecutionEnvironment {
		if self.0.len() < 1 {
			return ExecutionEnvironment::WasmtimeGeneric
		}
		// match ExecutorParam::decode(&mut &self.0[0].clone()[..]) {
		// 	Ok(ExecutorParam::Environment(v)) => v,
		// 	_ => ExecutionEnvironment::WasmtimeGeneric,
		// }
		if let ExecutorParam::Environment(env) = self.0[0] {
			env
		} else {
			ExecutionEnvironment::WasmtimeGeneric
		}
	}
}

impl Deref for ExecutorParams {
	// type Target = Vec<(u8, u64)>;
	type Target = Vec<ExecutorParam>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Default for ExecutorParams {
	fn default() -> Self {
		ExecutorParams(Vec::new())
	}
}

// impl From<&[(u8, u64)]> for ExecutorParams {
// 	fn from(arr: &[(u8, u64)]) -> Self {
// 		ExecutorParams(arr.into())
// 	}
// }

impl From<&[ExecutorParam]> for ExecutorParams {
	fn from(arr: &[ExecutorParam]) -> Self {
		ExecutorParams(arr.to_vec())
	}
}
