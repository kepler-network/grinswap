use super::multisig;
use super::types::Status;
use failure::Fail;
use grin_core::core::committed;
use grin_util::secp;
use libwallet;
use std::io;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
	#[fail(display = "Unexpected action")]
	UnexpectedAction,
	#[fail(display = "Unexpected network")]
	UnexpectedNetwork,
	#[fail(display = "Unexpected role")]
	UnexpectedRole,
	#[fail(display = "Unexpected status. Expected: _0, actual: _1")]
	UnexpectedStatus(Status, Status),
	#[fail(display = "Insufficient funds. Required: _0, available: _1")]
	InsufficientFunds(u64, u64),
	#[fail(display = "Unexpected message type")]
	UnexpectedMessageType,
	#[fail(display = "Unexpected secondary coin type")]
	UnexpectedCoinType,
	#[fail(display = "Version incompatibility")]
	IncompatibleVersion,
	#[fail(display = "Mismatch between swap and message IDs")]
	MismatchedId,
	#[fail(display = "Invalid lock height for lock tx")]
	InvalidLockHeightLockTx,
	#[fail(display = "Invalid lock height for refund tx")]
	InvalidLockHeightRefundTx,
	#[fail(display = "Invalid lock for secondary currency")]
	InvalidLockSecondary,
	#[fail(display = "Invalid adaptor signature")]
	InvalidAdaptorSignature,
	#[fail(display = "Secondary currency data complete")]
	SecondaryDataIncomplete,
	#[fail(display = "This function should only be called once")]
	OneShot,
	#[fail(display = "Swap is already finalized")]
	Finalized,
	#[fail(display = "Multisig: _0")]
	Multisig(multisig::ErrorKind),
	#[fail(display = "Keychain: _0")]
	Keychain(grin_keychain::Error),
	#[fail(display = "LibWallet: _0")]
	LibWallet(libwallet::ErrorKind),
	#[fail(display = "Secp: _0")]
	Secp(secp::Error),
	#[fail(display = "I/O error: _0")]
	IO(String),
	#[fail(display = "Serde error")]
	Serde,
	#[fail(display = "Rpc error: _0")]
	Rpc(&'static str),
	#[fail(display = "Node client error: _0")]
	NodeClient(String),
	#[fail(display = "_0")]
	Generic(String),
}

impl From<grin_keychain::Error> for ErrorKind {
	fn from(error: grin_keychain::Error) -> ErrorKind {
		ErrorKind::Keychain(error)
	}
}

impl From<multisig::ErrorKind> for ErrorKind {
	fn from(error: multisig::ErrorKind) -> ErrorKind {
		ErrorKind::Multisig(error)
	}
}

impl From<libwallet::Error> for ErrorKind {
	fn from(error: libwallet::Error) -> ErrorKind {
		ErrorKind::LibWallet(error.kind())
	}
}

impl From<secp::Error> for ErrorKind {
	fn from(error: secp::Error) -> ErrorKind {
		ErrorKind::Secp(error)
	}
}

impl From<io::Error> for ErrorKind {
	fn from(error: io::Error) -> ErrorKind {
		ErrorKind::IO(format!("{}", error))
	}
}

impl From<serde_json::Error> for ErrorKind {
	fn from(_error: serde_json::Error) -> ErrorKind {
		ErrorKind::Serde
	}
}

impl From<committed::Error> for ErrorKind {
	fn from(error: committed::Error) -> ErrorKind {
		match error {
			committed::Error::Keychain(e) => e.into(),
			committed::Error::Secp(e) => e.into(),
			e => ErrorKind::Generic(format!("{}", e)),
		}
	}
}
