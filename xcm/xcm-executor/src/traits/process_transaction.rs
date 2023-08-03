// Copyright 2020 Parity Technologies (UK) Ltd.
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

use xcm::latest::prelude::*;

/// Handles transactional processing of XCM instructions.
pub trait ProcessTransaction {
	/// Whether the XCM executor is transactional.
	const IS_TRANSACTIONAL: bool;
	/// The `process_transaction` method takes a closure as an argument.
	/// Responsible for processing an XCM instruction in a transaction.
	fn process_transaction<F>(f: F) -> Result<(), XcmError>
	where
		F: FnOnce() -> Result<(), XcmError>;
}

impl ProcessTransaction for () {
	const IS_TRANSACTIONAL: bool = false;
	fn process_transaction<F>(f: F) -> Result<(), XcmError>
	where
		F: FnOnce() -> Result<(), XcmError>,
	{
		f()
	}
}