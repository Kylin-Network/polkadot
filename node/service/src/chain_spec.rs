// Copyright 2017-2020 Parity Technologies (UK) Ltd.
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

//! Polkadot chain configurations.

use beefy_primitives::crypto::AuthorityId as BeefyId;
use grandpa::AuthorityId as GrandpaId;
#[cfg(feature = "kusama-native")]
use kusama_runtime as kusama;
#[cfg(feature = "kusama-native")]
use kusama_runtime::constants::currency::UNITS as KSM;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
#[cfg(feature = "polkadot-native")]
use polkadot::constants::currency::UNITS as DOT;
use polkadot_primitives::v1::{AccountId, AccountPublic, AssignmentId, ValidatorId};
#[cfg(feature = "polkadot-native")]
use polkadot_runtime as polkadot;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;

#[cfg(feature = "rococo-native")]
use rococo_runtime as rococo;
#[cfg(feature = "rococo-native")]
use rococo_runtime::constants::currency::UNITS as ROC;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;
#[cfg(feature = "westend-native")]
use westend_runtime as westend;
#[cfg(feature = "westend-native")]
use westend_runtime::constants::currency::UNITS as WND;

#[cfg(feature = "polkadot-native")]
const POLKADOT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[cfg(feature = "kusama-native")]
const KUSAMA_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[cfg(feature = "westend-native")]
const WESTEND_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[cfg(feature = "rococo-native")]
const ROCOCO_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dot";
const CHACHACHA_PROTOCOL_ID: &str = "chachacha";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::v1::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::v1::Block>,
	/// The light sync state.
	///
	/// This value will be set by the `sync-state rpc` implementation.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// The `ChainSpec` parameterized for the polkadot runtime.
#[cfg(feature = "polkadot-native")]
pub type PolkadotChainSpec = service::GenericChainSpec<polkadot::GenesisConfig, Extensions>;

// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = service::GenericChainSpec<(), Extensions>;

// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "polkadot-native"))]
pub type PolkadotChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the kusama runtime.
#[cfg(feature = "kusama-native")]
pub type KusamaChainSpec = service::GenericChainSpec<kusama::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the kusama runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "kusama-native"))]
pub type KusamaChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the westend runtime.
#[cfg(feature = "westend-native")]
pub type WestendChainSpec = service::GenericChainSpec<westend::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the westend runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "westend-native"))]
pub type WestendChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the rococo runtime.
#[cfg(feature = "rococo-native")]
pub type RococoChainSpec = service::GenericChainSpec<RococoGenesisExt, Extensions>;

/// The `ChainSpec` parameterized for the rococo runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "rococo-native"))]
pub type RococoChainSpec = DummyChainSpec;

/// Extension for the Rococo genesis config to support a custom changes to the genesis state.
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg(feature = "rococo-native")]
pub struct RococoGenesisExt {
	/// The runtime genesis config.
	runtime_genesis_config: rococo::GenesisConfig,
	/// The session length in blocks.
	///
	/// If `None` is supplied, the default value is used.
	session_length_in_blocks: Option<u32>,
}

#[cfg(feature = "rococo-native")]
impl sp_runtime::BuildStorage for RococoGenesisExt {
	fn assimilate_storage(&self, storage: &mut sp_core::storage::Storage) -> Result<(), String> {
		sp_state_machine::BasicExternalities::execute_with_storage(storage, || {
			if let Some(length) = self.session_length_in_blocks.as_ref() {
				rococo::constants::time::EpochDurationInBlocks::set(length);
			}
		});
		self.runtime_genesis_config.assimilate_storage(storage)
	}
}

pub fn polkadot_config() -> Result<PolkadotChainSpec, String> {
	PolkadotChainSpec::from_json_bytes(&include_bytes!("../res/polkadot.json")[..])
}

pub fn kusama_config() -> Result<KusamaChainSpec, String> {
	KusamaChainSpec::from_json_bytes(&include_bytes!("../res/kusama.json")[..])
}

pub fn westend_config() -> Result<WestendChainSpec, String> {
	WestendChainSpec::from_json_bytes(&include_bytes!("../res/westend.json")[..])
}

pub fn rococo_config() -> Result<RococoChainSpec, String> {
	RococoChainSpec::from_json_bytes(&include_bytes!("../res/rococo-chachacha.json")[..])
}

/// This is a temporary testnet that uses the same runtime as rococo.
pub fn wococo_config() -> Result<RococoChainSpec, String> {
	RococoChainSpec::from_json_bytes(&include_bytes!("../res/wococo.json")[..])
}

/// The default parachains host configuration.
#[cfg(any(
	feature = "rococo-native",
	feature = "kusama-native",
	feature = "westend-native",
	feature = "polkadot-native"
))]
fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<
	polkadot_primitives::v1::BlockNumber,
> {
	use polkadot_primitives::v1::{MAX_CODE_SIZE, MAX_POV_SIZE};

	polkadot_runtime_parachains::configuration::HostConfiguration {
		validation_upgrade_frequency: 1u32,
		validation_upgrade_delay: 1,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		chain_availability_period: 4,
		thread_availability_period: 4,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024,
		// this is approximatelly 4ms.
		//
		// Same as `4 * frame_support::weights::WEIGHT_PER_MILLIS`. We don't bother with
		// an import since that's a made up number and should be replaced with a constant
		// obtained by benchmarking anyway.
		ump_service_total_weight: 4 * 1_000_000_000,
		max_upward_message_size: 1024 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_max_parathread_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_parathread_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		..Default::default()
	}
}

#[cfg(feature = "polkadot-native")]
fn polkadot_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> polkadot::SessionKeys {
	polkadot::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "kusama-native")]
fn kusama_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> kusama::SessionKeys {
	kusama::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "westend-native")]
fn westend_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> westend::SessionKeys {
	westend::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "rococo-native")]
fn rococo_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
	beefy: BeefyId,
) -> rococo_runtime::SessionKeys {
	rococo_runtime::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
		beefy,
	}
}

#[cfg(feature = "polkadot-native")]
fn polkadot_staging_testnet_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![];

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	const STASH: u128 = 100 * DOT;

	polkadot::GenesisConfig {
		system: polkadot::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: polkadot::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: polkadot::IndicesConfig { indices: vec![] },
		session: polkadot::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: polkadot::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, polkadot::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: Default::default(),
		council: polkadot::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: polkadot::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: polkadot::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(polkadot::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: polkadot::AuthorityDiscoveryConfig { keys: vec![] },
		claims: polkadot::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: polkadot::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		configuration: polkadot::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
	}
}

#[cfg(feature = "westend-native")]
fn westend_staging_testnet_config_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5DaVh5WRfazkGaKhx1jUu6hjz7EmRe4dtW6PKeVLim84KLe8
		hex!["42f4a4b3e0a89c835ee696205caa90dd85c8ea1d7364b646328ee919a6b2fc1e"].into(),
	];
	// SECRET='...' ./scripts/prepare-test-net.sh 4
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			//5ERCqy118nnXDai8g4t3MjdX7ZC5PrQzQpe9vwex5cELWqbt
			hex!["681af4f93073484e1acd6b27395d0d258f1a6b158c808846c8fd05ee2435056e"].into(),
			//5GTS114cfQNBgpQULhMaNCPXGds6NokegCnikxDe1vqANhtn
			hex!["c2463372598ebabd21ee5bc33e1d7e77f391d2df29ce2fbe6bed0d13be629a45"].into(),
			//5FhGbceKeH7fuGogcBwd28ZCkAwDGYBADCTeHiYrvx2ztyRd
			hex!["a097bfc6a33499ed843b711f52f523f8a7174f798a9f98620e52f4170dbe2948"]
				.unchecked_into(),
			//5Es7nDkJt2by5qVCCD7PZJdp76KJw1LdRCiNst5S5f4eecnz
			hex!["7bde49dda82c2c9f082b807ef3ceebff96437d67b3e630c584db7a220ecafacf"]
				.unchecked_into(),
			//5D4e8zRjaYzFamqChGPPtu26PcKbKgUrhb7WqcNbKa2RDFUR
			hex!["2c2fb730a7d9138e6d62fcf516f9ecc2d712af3f2f03ca330c9564b8c0c1bb33"]
				.unchecked_into(),
			//5DD3JY5ENkjcgVFbVSgUbZv7WmrnyJ8bxxu56ee6hZFiRdnh
			hex!["3297a8622988cc23dd9c131e3fb8746d49e007f6e58a81d43420cd539e250e4c"]
				.unchecked_into(),
			//5Gpodowhud8FG9xENXR5YwTFbUAWyoEtw7sYFytFsG4z7SU6
			hex!["d2932edf775088bd088dc5a112ad867c24cc95858f77f8a1ab014de8d4f96a3f"]
				.unchecked_into(),
			//5GUMj8tnjL3PJZgXoiWtgLCaMVNHBNeSeTqDsvcxmaVAjKn9
			hex!["c2fb0f74591a00555a292bc4882d3158bafc4c632124cb60681f164ef81bcf72"]
				.unchecked_into(),
		),
		(
			//5HgDCznTkHKUjzPkQoTZGWbvbyqB7sqHDBPDKdF1FyVYM7Er
			hex!["f8418f189f84814fd40cc1b2e90873e72ea789487f3b98ed42811ba76d10fc37"].into(),
			//5GQTryeFwuvgmZ2tH5ZeAKZHRM9ch5WGVGo6ND9P8f9uMsNY
			hex!["c002bb4af4a1bd2f33d104aef8a41878fe1ac94ba007029c4dfdefa8b698d043"].into(),
			//5C7YkWSVH1zrpsE5KwW1ua1qatyphzYxiZrL24mjkxz7mUbn
			hex!["022b14fbcf65a93b81f453105b9892c3fc4aa74c22c53b4abab019e1d58fbd41"]
				.unchecked_into(),
			//5GwFC6Tmg4fhj4PxSqHycgJxi3PDfnC9RGDsNHoRwAvXvpnZ
			hex!["d77cafd3b32c8b52b0e2780a586a6e527c94f1bdec117c4e4acb0a491461ffa3"]
				.unchecked_into(),
			//5DSVrGURuDuh8Luzo8FYq7o2NWiUSLSN6QAVNrj9BtswWH6R
			hex!["3cdb36a5a14715999faffd06c5b9e5dcdc24d4b46bc3e4df1aaad266112a7b49"]
				.unchecked_into(),
			//5DLEG2AupawCXGwhJtrzBRc3zAhuP8V662dDrUTzAsCiB9Ec
			hex!["38134245c9919ecb20bf2eedbe943b69ba92ceb9eb5477b92b0afd3cb6ce2858"]
				.unchecked_into(),
			//5D83o9fDgnHxaKPkSx59hk8zYzqcgzN2mrf7cp8fiVEi7V4E
			hex!["2ec917690dc1d676002e3504c530b2595490aa5a4603d9cc579b9485b8d0d854"]
				.unchecked_into(),
			//5DwBJquZgncRWXFxj2ydbF8LBUPPUbiq86sXWXgm8Z38m8L2
			hex!["52bae9b8dedb8058dda93ec6f57d7e5a517c4c9f002a4636fada70fed0acf376"]
				.unchecked_into(),
		),
		(
			//5DMHpkRpQV7NWJFfn2zQxCLiAKv7R12PWFRPHKKk5X3JkYfP
			hex!["38e280b35d08db46019a210a944e4b7177665232ab679df12d6a8bbb317a2276"].into(),
			//5FbJpSHmFDe5FN3DVGe1R345ZePL9nhcC9V2Cczxo7q8q6rN
			hex!["9c0bc0e2469924d718ae683737f818a47c46b0612376ecca06a2ac059fe1f870"].into(),
			//5E5Pm3Udzxy26KGkLE5pc8JPfQrvkYHiaXWtuEfmQsBSgep9
			hex!["58fecadc2df8182a27e999e7e1fd7c99f8ec18f2a81f9a0db38b3653613f3f4d"]
				.unchecked_into(),
			//5FxcystSLHtaWoy2HEgRNerj9PrUs452B6AvHVnQZm5ZQmqE
			hex!["ac4d0c5e8f8486de05135c10a707f58aa29126d5eb28fdaaba00f9a505f5249d"]
				.unchecked_into(),
			//5E7KqVXaVGuAqiqMigpuH8oXHLVh4tmijmpJABLYANpjMkem
			hex!["5a781385a0235fe8594dd101ec55ef9ba01883f8563a0cdd37b89e0303f6a578"]
				.unchecked_into(),
			//5H9AybjkpyZ79yN5nHuBqs6RKuZPgM7aAVVvTQsDFovgXb2A
			hex!["e09570f62a062450d4406b4eb43e7f775ff954e37606646cd590d1818189501f"]
				.unchecked_into(),
			//5Ccgs7VwJKBawMbwMENDmj2eFAxhFdGksVHdk8aTAf4w7xox
			hex!["1864832dae34df30846d5cc65973f58a2d01b337d094b1284ec3466ecc90251d"]
				.unchecked_into(),
			//5EsSaZZ7niJs7hmAtp4QeK19AcAuTp7WXB7N7gRipVooerq4
			hex!["7c1d92535e6d94e21cffea6633a855a7e3c9684cd2f209e5ddbdeaf5111e395b"]
				.unchecked_into(),
		),
		(
			//5Ea11qhmGRntQ7pyEkEydbwxvfrYwGMKW6rPERU4UiSBB6rd
			hex!["6ed057d2c833c45629de2f14b9f6ce6df1edbf9421b7a638e1fb4828c2bd2651"].into(),
			//5CZomCZwPB78BZMZsCiy7WSpkpHhdrN8QTSyjcK3FFEZHBor
			hex!["1631ff446b3534d031adfc37b7f7aed26d2a6b3938d10496aab3345c54707429"].into(),
			//5CSM6vppouFHzAVPkVFWN76DPRUG7B9qwJe892ccfSfJ8M5f
			hex!["108188c43a7521e1abe737b343341c2179a3a89626c7b017c09a5b10df6f1c42"]
				.unchecked_into(),
			//5GwkG4std9KcjYi3ThSC7QWfhqokmYVvWEqTU9h7iswjhLnr
			hex!["d7de8a43f7ee49fa3b3aaf32fb12617ec9ff7b246a46ab14e9c9d259261117fa"]
				.unchecked_into(),
			//5CoUk3wrCGJAWbiJEcsVjYhnd2JAHvR59jBRbSw77YrBtRL1
			hex!["209f680bc501f9b59358efe3636c51fd61238a8659bac146db909aea2595284b"]
				.unchecked_into(),
			//5EcSu96wprFM7G2HfJTjYu8kMParnYGznSUNTsoEKXywEsgG
			hex!["70adf80395b3f59e4cab5d9da66d5a286a0b6e138652a06f72542e46912df922"]
				.unchecked_into(),
			//5Ge3sjpD43Cuy7rNoJQmE9WctgCn6Faw89Pe7xPs3i55eHwJ
			hex!["ca5f6b970b373b303f64801a0c2cadc4fc05272c6047a2560a27d0c65589ca1d"]
				.unchecked_into(),
			//5EFcjHLvB2z5vd5g63n4gABmhzP5iPsKvTwd8sjfvTehNNrk
			hex!["60cae7fa5a079d9fc8061d715fbcc35ef57c3b00005694c2badce22dcc5a9f1b"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * WND;
	const STASH: u128 = 100 * WND;

	westend::GenesisConfig {
		system: westend::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: westend::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: westend::IndicesConfig { indices: vec![] },
		session: westend::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						westend_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: westend::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, westend::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: westend::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(westend::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: westend::AuthorityDiscoveryConfig { keys: vec![] },
		vesting: westend::VestingConfig { vesting: vec![] },
		sudo: westend::SudoConfig { key: endowed_accounts[0].clone() },
		configuration: westend::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		registrar: westend_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v1::LOWEST_PUBLIC_ID,
		},
	}
}

#[cfg(feature = "kusama-native")]
fn kusama_staging_testnet_config_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5CVFESwfkk7NmhQ6FwHCM9roBvr9BGa4vJHFYU8DnGQxrXvz
		hex!["12b782529c22032ed4694e0f6e7d486be7daa6d12088f6bc74d593b3900b8438"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in para_validator para_assignment; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5DD7Q4VEfPTLEdn11CnThoHT5f9xKCrnofWJL5SsvpTghaAT
			hex!["32a5718e87d16071756d4b1370c411bbbb947eb62f0e6e0b937d5cbfc0ea633b"].into(),
			// 5GNzaEqhrZAtUQhbMe2gn9jBuNWfamWFZHULryFwBUXyd1cG
			hex!["bee39fe862c85c91aaf343e130d30b643c6ea0b4406a980206f1df8331f7093b"].into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5EjvdwATjyFFikdZibVvx1q5uBHhphS2Mnsq5c7yfaYK25vm
			hex!["76620f7c98bce8619979c2b58cf2b0aff71824126d2b039358729dad993223db"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
		),
		(
			// 5G9VGb8ESBeS8Ca4or43RfhShzk9y7T5iTmxHk5RJsjZwsRx
			hex!["b496c98a405ceab59b9e970e59ef61acd7765a19b704e02ab06c1cdfe171e40f"].into(),
			// 5F7V9Y5FcxKXe1aroqvPeRiUmmeQwTFcL3u9rrPXcMuMiCNx
			hex!["86d3a7571dd60139d297e55d8238d0c977b2e208c5af088f7f0136b565b0c103"].into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5HBDAaybNqjmY7ww8ZcZZY1L5LHxvpnyfqJwoB7HhR6raTmG
			hex!["e2234d661bee4a04c38392c75d1566200aa9e6ae44dd98ee8765e4cc9af63cb7"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
		),
		(
			// 5FzwpgGvk2kk9agow6KsywLYcPzjYc8suKej2bne5G5b9YU3
			hex!["ae12f70078a22882bf5135d134468f77301927aa67c376e8c55b7ff127ace115"].into(),
			// 5EqoZhVC2BcsM4WjvZNidu2muKAbu5THQTBKe3EjvxXkdP7A
			hex!["7addb914ec8486bbc60643d2647685dcc06373401fa80e09813b630c5831d54b"].into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5E8ULLQrDAtWhfnVfZmX41Yux86zNAwVJYguWJZVWrJvdhBe
			hex!["5b57ed1443c8967f461db1f6eb2ada24794d163a668f1cf9d9ce3235dfad8799"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
		),
		(
			// 5CFj6Kg9rmVn1vrqpyjau2ztyBzKeVdRKwNPiA3tqhB5HPqq
			hex!["0867dbb49721126df589db100dda728dc3b475cbf414dad8f72a1d5e84897252"].into(),
			// 5CwQXP6nvWzigFqNhh2jvCaW9zWVzkdveCJY3tz2MhXMjTon
			hex!["26ab2b4b2eba2263b1e55ceb48f687bb0018130a88df0712fbdaf6a347d50e2a"].into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5HGLmrZsiTFTPp3QoS1W8w9NxByt8PVq79reqvdxNcQkByqK
			hex!["e60d23f49e93c1c1f2d7c115957df5bbd7faf5ebf138d1e9d02e8b39a1f63df0"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	kusama::GenesisConfig {
		system: kusama::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: kusama::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: kusama::IndicesConfig { indices: vec![] },
		session: kusama::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						kusama_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: kusama::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, kusama::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: Default::default(),
		council: kusama::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: kusama::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: kusama::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(kusama::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: kusama::AuthorityDiscoveryConfig { keys: vec![] },
		claims: kusama::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: kusama::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		configuration: kusama::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		gilt: Default::default(),
		paras: Default::default(),
	}
}

#[cfg(feature = "rococo-native")]
fn rococo_staging_testnet_config_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5Ft2Noj74oqjtRAUiPuLVtMJmf917G7AQ3fHGkBn3aC8Ftat
		hex!["a8cb93b2be45d72e6901fa2ea9a51f3e1a42ad8ec5f48a580407d76d9921f675"].into(),
		// 5FH6F68v1eopRVP4ogzHe2typpPBWjFheUqqHenknTi2w9hp
		hex!["8e27014e1ecf8f069081ae73d771f39f0113bc1cdc753c6a6da114a48d9b5a05"].into(),
	];

	// ./scripts/prepare-test-net.sh 8
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)> = vec![
		(
			//5DUWJekG4F2YPaRkf8SXDwdpPnAeFLEZhfW2bwNiT7qXah64
			hex!["3e633c4fcec666479afb69c29c27e9d8407eefbfacb8b2ea27d178b9400a5170"].into(),
			//5GZA5ctKST8SqZgawjpVFgpWaxEFS3mwRF4FuzGhSeqrMZJ7
			hex!["c6a414a1d1551b7f7dfdca752833b91c8136fbb0fef5fde5188fa00e6d8bb02d"].into(),
			//5GpFyTqFQpi4AUAL5f4MH63c5E4dnyo7HDTvGgvmZ7iqGbSh
			hex!["d2289a275a32e47d99e12f325267ac0ffe1d0783c96f41e8c1328713a1781330"].unchecked_into(),
			//5G3Ha2jVf1PKDLX7QCmrFBmE1HMTBws5ZHGznuU4rmuSZPMh
			hex!["afdbee794c6b74c04ad7352a61f1f5d9b11687175d017d38c6c8c20b64176faa"].unchecked_into(),
			//5G8jjSYJFqf2rCuVL6VHL3B4Kopp6pQPnmBAjXPMn5zEsNQi
			hex!["b4043a81bf19ab0e28c62b9cabd79932a59ac710e43f52b8016a7d1c82666d1c"].unchecked_into(),
			//5CCxn5oKMhgsgd7dMPGyRxp7syFySfMt2hKZgyb921Ax5uxk
			hex!["064c310d8495325895a73216972aaeac743b661f9c46cf3e3b045ffc81d60628"].unchecked_into(),
			//5Dvg5r1zsfPCxG3V3tC2r5yp7RtfVJizvCfZxTYTSFj2uMCn
			hex!["5258892ece3d0949394d8f592c4cdd9fcc94962e69191fd95f6e0f4146795719"].unchecked_into(),
			//5FkMcDTF8bhGV5NgR4J3iw9AzaSXXRcGvmK3MTDvMpk5KWwH
			hex!["a2f25a071c0a701b8bd41fcab21d38462fbf9493b1ab78991c6f0624e2e34f1c"].unchecked_into(),
			//5Di6QY2YRwXFcXthNeEKz3NYPboZRWTExhRWcGsNscj4pTNt
			hex!["02a58557e57610b4eabd969084da9ecb915e3c76a6d50811d5b42ab1f32bfacedf"].unchecked_into(),
		),
		(
			//5H8ufmHnZquZ84d55vav3ovucFoJUkgB28PfFJo9TSS1eqq9
			hex!["e061e95e9e047d79d1495810d938d8131339f4af0e021500bdbedd7afeb3b56b"].into(),
			//5GjbjmMjDF5PC1arpjZbsRxAuPYadUWyM6jGuRVZaaZoZzwN
			hex!["ce9ae7f7ce367ee42a708e70453e8fcdabae2076ac46536921793d6737578563"].into(),
			//5FezeW8NHzz9qzCafjTovYdUpqx2LF1fQbvWKtZGn5vx8k93
			hex!["9edb90ab3c60daa595a8987246cd4d64bd3e196b617c008ea3b56bdc9dd1cf65"].unchecked_into(),
			//5FfPnW2usQXzPynHN6G7QH4NMgg14SeH1EaoJUNTTNQAdagV
			hex!["9f297448ec202f6af48cb26aa4c0446a46ff0452de84e11f6286fb3532d9a8a3"].unchecked_into(),
			//5CuAvpkprBhBuD3ADVKVmFFtS7SAwKWi4nk6ip3dhym5ZSGg
			hex!["24f6e92314e5fa32d15c91f615f1f91555d674b19dcb3d4b25e4d87961eca51c"].unchecked_into(),
			//5DVGggRbS8yz7Ymu14X96vbS4Qvg3VX5ZQGwLRChejfgNSGL
			hex!["3ef8a1a0031cc9768b95385e7a9413698519e81f775198b54d930d3f23b83736"].unchecked_into(),
			//5H6je32nFSz1JuwTpyjx7rdHGaPhYj79TyK3Z6SJ9Xw35j28
			hex!["deb9a84f83072dd64ccddbb24c00fe08aeac23f214adcdeb0c6d6a80b7629f24"].unchecked_into(),
			//5HmSQTBi8c6sitSEwkaXcvvaHFCbJhr3BRLFviXZRLJk7nGb
			hex!["fc3e36769c550f99bdaa38b8574a9a8b4d553bb2081628c05bcc67767b752534"].unchecked_into(),
			//5HVeJtYkGQpkBQGDGoqrfLyZ6TU8YmRmXpw8SYSBmCTuRUqj
			hex!["026647ae3357a10afad79367654c851d1d5b471ce16be66eee1c65921a9ba63b24"].unchecked_into(),
		),
		(
			//5GRLDWnMSYog7Pdh9z6muiko8vQXw5ftApMAdJetiFs6fz7m
			hex!["c0ac3d12f037ce7029f4ed937f7e4a0a7dd448017947b70627d6dfb44b7f740f"].into(),
			//5HYWwp1KrhFUabhShtb4WHR5CqwjyiQ4txrnDd3YuNJZi3RD
			hex!["f2634ec910957287c04df0be4afbd79d6ddde6ae131d869867275d96dd75843b"].into(),
			//5D7HfSVAXa4S7L6oVQPQ1Xki8wc8BjWAk4Dr45avUhQd6gMx
			hex!["2e3486da0704a2e11407763184fe4ef2d4bf41daa9b79217db64abb3fd37d150"].unchecked_into(),
			//5FL8g3iQxEu4tMuvX5HjoLjVU4teExrKKyahoZ6dY4Wj8SvE
			hex!["9078ec27929d748745f4b46a979e72766d5187348c5f5674260f17c2075ff5cc"].unchecked_into(),
			//5FCHejTz8tKT32vqsNvGSqKNa6pvMSjogubZup62t4rR7FAW
			hex!["8a7d2dfd21927d609985d02168b1268cd859dc9da987cbf57fc96c8ed5b3db0e"].unchecked_into(),
			//5FTixuzvuNjufPd4vhAndMjkwy4nD6wZr1GTRS7kzxM1nysJ
			hex!["964312cea3bfbee6179d5faa3124338debbb0619be45c9b15c064e66ab45b812"].unchecked_into(),
			//5DAKGyF4gFUrsSRTMyPYnM4ehBtAJsQcKneg61azDWVKxkcU
			hex!["3083b1026ef95b0df745fec5992ecd378ab3fd12c0935feb817b5755fdd81f6a"].unchecked_into(),
			//5FNcHjvu65fNj1Mhn4DGRWMMbwsWFkTy5K7cNCCxtixvbNUk
			hex!["925c600154e3e8a494ee1bb9c387236e82c511e315164e0cafa181b9edba2e08"].unchecked_into(),
			//5Fx5x2g94G3P3aVw4ckaB8XsdoqHnJx21Ush1UY9hRvU7o4r
			hex!["03deba46a2167445bb80c47d16d4148ce254d0a973ee0bc27fee045b6090d502c6"].unchecked_into(),
		),
		(
			//5HHZVXqJPHtPx533pjRJNURVF2F1oGSWCfWEvwFFLgFaWJba
			hex!["e6fb334bcc25dfb0c5bcb8a0948e00c7eb494d60ec2f9228aecea93c7c095309"].into(),
			//5FsYsf6ikSJEHpT8Fy7erigxMQCnQWp5dLHRZDdyVsea4w6r
			hex!["a86efebc4f1133644e4c79b91ad9e3064bc9f37902d4f93d89c0a4cc95cbb306"].into(),
			//5DVD6nVKcksJu2XAy14nDW3x1WwJdftNevrEQHLkVnst78w4
			hex!["3eec90aed4af142912b5da34e68fde0dd77e3b8dc2b13a19e92cda9b00e2082b"].unchecked_into(),
			//5H99Z1Ntt6NxEKgLRh6F1h7sF36zTfRgtpaHq1QQKaaeEDpi
			hex!["e090a5ca803c2b1de14690c097095828eed80d523b3813ee8eb2fbeeb9be4823"].unchecked_into(),
			//5CLoemuHQTLReUvTNtrTa7peBxbBeuYHS6PXsog2etB8ro8B
			hex!["0c476f17fcc98581054db0993edb78082721e90070baf323c4a9a380f1e65b31"].unchecked_into(),
			//5Gba4TnydtNukSsUfG82xRpykYn3KmLjW2hvGxXvAADcJtBn
			hex!["c87b4ba3b4b640d1beeaa8338e8552eeadd0756dc65b2f9dad376d45dd94fe20"].unchecked_into(),
			//5GWnsBgeRup7SqGyyvXf4CGQs267GGbuhZcFN5Dohrzbq2kD
			hex!["c4d62d008a12724383d2439ddc81423b5041e0908aac242690b201f64a4c3848"].unchecked_into(),
			//5CUNZCVuFQ8rm1ueyqtsbfrVyBvtFhGF1Uhi6DnpQ6D1mE5z
			hex!["120cea83777f792ada9509cf2fed202f0d48c729ecdfbdff458b487add741470"].unchecked_into(),
			//5FjtkURzTFGST8JYEmmnnwwjmYuD87pUoSoCN91K8SDnPu5t
			hex!["039b90c14351ab0436b6bd4ff2b63ef85f7423c226b18caac49ae2cd3244f95b1a"].unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * ROC;
	const STASH: u128 = 100 * ROC;

	rococo_runtime::GenesisConfig {
		system: rococo_runtime::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: rococo_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		beefy: Default::default(),
		indices: rococo_runtime::IndicesConfig { indices: vec![] },
		session: rococo_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						rococo_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
							x.8.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		babe: rococo_runtime::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(rococo_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		collective: Default::default(),
		membership: Default::default(),
		authority_discovery: rococo_runtime::AuthorityDiscoveryConfig { keys: vec![] },
		sudo: rococo_runtime::SudoConfig { key: endowed_accounts[0].clone() },
		paras: rococo_runtime::ParasConfig { paras: vec![] },
		hrmp: Default::default(),
		configuration: rococo_runtime::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		registrar: rococo_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v1::LOWEST_PUBLIC_ID,
		},
		// bridge_rococo_grandpa: rococo_runtime::BridgeRococoGrandpaConfig {
		// 	owner: Some(endowed_accounts[0].clone()),
		// 	..Default::default()
		// },
		// bridge_wococo_grandpa: rococo_runtime::BridgeWococoGrandpaConfig {
		// 	owner: Some(endowed_accounts[0].clone()),
		// 	..Default::default()
		// },
		// bridge_rococo_messages: rococo_runtime::BridgeRococoMessagesConfig {
		// 	owner: Some(endowed_accounts[0].clone()),
		// 	..Default::default()
		// },
		// bridge_wococo_messages: rococo_runtime::BridgeWococoMessagesConfig {
		// 	owner: Some(endowed_accounts[0].clone()),
		// 	..Default::default()
		// },
	}
}

/// Polkadot staging testnet config.
#[cfg(feature = "polkadot-native")]
pub fn polkadot_staging_testnet_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;
	let boot_nodes = vec![];

	Ok(PolkadotChainSpec::from_genesis(
		"Polkadot Staging Testnet",
		"polkadot_staging_testnet",
		ChainType::Live,
		move || polkadot_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Polkadot Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Staging testnet config.
#[cfg(feature = "kusama-native")]
pub fn kusama_staging_testnet_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;
	let boot_nodes = vec![];

	Ok(KusamaChainSpec::from_genesis(
		"Kusama Staging Testnet",
		"kusama_staging_testnet",
		ChainType::Live,
		move || kusama_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(KUSAMA_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Kusama Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Westend staging testnet config.
#[cfg(feature = "westend-native")]
pub fn westend_staging_testnet_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;
	let boot_nodes = vec![];

	Ok(WestendChainSpec::from_genesis(
		"Westend Staging Testnet",
		"westend_staging_testnet",
		ChainType::Live,
		move || westend_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(WESTEND_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Westend Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Rococo staging testnet config.
#[cfg(feature = "rococo-native")]
pub fn rococo_staging_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;
	let boot_nodes = vec![];

	Ok(RococoChainSpec::from_genesis(
		"Chachacha Staging Testnet",
		"rococo_chachacha_staging_testnet",
		ChainType::Live,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_staging_testnet_config_genesis(wasm_binary),
			session_length_in_blocks: Some(100),
		},
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(ROCOCO_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Rococo Staging telemetry url is valid; qed"),
		),
		Some(CHACHACHA_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	]
}

/// Helper function to create polkadot `GenesisConfig` for testing
#[cfg(feature = "polkadot-native")]
pub fn polkadot_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> polkadot::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	const STASH: u128 = 100 * DOT;

	polkadot::GenesisConfig {
		system: polkadot::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		indices: polkadot::IndicesConfig { indices: vec![] },
		balances: polkadot::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: polkadot::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: polkadot::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, polkadot::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: polkadot::DemocracyConfig::default(),
		council: polkadot::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: polkadot::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: polkadot::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(polkadot::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: polkadot::AuthorityDiscoveryConfig { keys: vec![] },
		claims: polkadot::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: polkadot::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		configuration: polkadot::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
	}
}

/// Helper function to create kusama `GenesisConfig` for testing
#[cfg(feature = "kusama-native")]
pub fn kusama_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> kusama::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	kusama::GenesisConfig {
		system: kusama::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		indices: kusama::IndicesConfig { indices: vec![] },
		balances: kusama::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: kusama::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						kusama_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: kusama::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, kusama::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: kusama::DemocracyConfig::default(),
		council: kusama::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: kusama::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: kusama::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(kusama::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: kusama::AuthorityDiscoveryConfig { keys: vec![] },
		claims: kusama::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: kusama::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		configuration: kusama::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		gilt: Default::default(),
		paras: Default::default(),
	}
}

/// Helper function to create westend `GenesisConfig` for testing
#[cfg(feature = "westend-native")]
pub fn westend_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> westend::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOT;
	const STASH: u128 = 100 * DOT;

	westend::GenesisConfig {
		system: westend::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		indices: westend::IndicesConfig { indices: vec![] },
		balances: westend::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: westend::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						westend_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: westend::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, westend::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: westend::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(westend::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: westend::AuthorityDiscoveryConfig { keys: vec![] },
		vesting: westend::VestingConfig { vesting: vec![] },
		sudo: westend::SudoConfig { key: root_key },
		configuration: westend::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		registrar: westend_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v1::LOWEST_PUBLIC_ID,
		},
	}
}

/// Helper function to create rococo `GenesisConfig` for testing
#[cfg(feature = "rococo-native")]
pub fn rococo_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> rococo_runtime::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOT;

	rococo_runtime::GenesisConfig {
		system: rococo_runtime::SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		beefy: Default::default(),
		indices: rococo_runtime::IndicesConfig { indices: vec![] },
		balances: rococo_runtime::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: rococo_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						rococo_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
							x.8.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		babe: rococo_runtime::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(rococo_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		collective: Default::default(),
		membership: Default::default(),
		authority_discovery: rococo_runtime::AuthorityDiscoveryConfig { keys: vec![] },
		sudo: rococo_runtime::SudoConfig { key: root_key.clone() },
		configuration: rococo_runtime::ConfigurationConfig {
			config: polkadot_runtime_parachains::configuration::HostConfiguration {
				max_validators_per_core: Some(1),
				..default_parachains_host_configuration()
			},
		},
		hrmp: Default::default(),
		paras: rococo_runtime::ParasConfig { paras: vec![] },
		registrar: rococo_runtime::RegistrarConfig {
			next_free_para_id: polkadot_primitives::v1::LOWEST_PUBLIC_ID,
		},
		// bridge_rococo_grandpa: rococo_runtime::BridgeRococoGrandpaConfig {
		// 	owner: Some(root_key.clone()),
		// 	..Default::default()
		// },
		// bridge_wococo_grandpa: rococo_runtime::BridgeWococoGrandpaConfig {
		// 	owner: Some(root_key.clone()),
		// 	..Default::default()
		// },
		// bridge_rococo_messages: rococo_runtime::BridgeRococoMessagesConfig {
		// 	owner: Some(root_key.clone()),
		// 	..Default::default()
		// },
		// bridge_wococo_messages: rococo_runtime::BridgeWococoMessagesConfig {
		// 	owner: Some(root_key.clone()),
		// 	..Default::default()
		// },
	}
}

#[cfg(feature = "polkadot-native")]
fn polkadot_development_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	polkadot_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "kusama-native")]
fn kusama_development_config_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	kusama_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "westend-native")]
fn westend_development_config_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	westend_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "rococo-native")]
fn rococo_development_config_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot development config (single validator Alice)
#[cfg(feature = "polkadot-native")]
pub fn polkadot_development_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

	Ok(PolkadotChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || polkadot_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Kusama development config (single validator Alice)
#[cfg(feature = "kusama-native")]
pub fn kusama_development_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;

	Ok(KusamaChainSpec::from_genesis(
		"Development",
		"kusama_dev",
		ChainType::Development,
		move || kusama_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Westend development config (single validator Alice)
#[cfg(feature = "westend-native")]
pub fn westend_development_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;

	Ok(WestendChainSpec::from_genesis(
		"Development",
		"westend_dev",
		ChainType::Development,
		move || westend_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Rococo development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn rococo_development_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Development",
		"rococo_dev",
		ChainType::Development,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Wococo development config (single validator Alice)
#[cfg(feature = "rococo-native")]
pub fn wococo_development_config() -> Result<RococoChainSpec, String> {
	const WOCOCO_DEV_PROTOCOL_ID: &str = "woco";
	let wasm_binary = rococo::WASM_BINARY.ok_or("Wococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Development",
		"wococo_dev",
		ChainType::Development,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(WOCOCO_DEV_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

#[cfg(feature = "polkadot-native")]
fn polkadot_local_testnet_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
	polkadot_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "polkadot-native")]
pub fn polkadot_local_testnet_config() -> Result<PolkadotChainSpec, String> {
	let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

	Ok(PolkadotChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || polkadot_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

#[cfg(feature = "kusama-native")]
fn kusama_local_testnet_genesis(wasm_binary: &[u8]) -> kusama::GenesisConfig {
	kusama_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Kusama local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "kusama-native")]
pub fn kusama_local_testnet_config() -> Result<KusamaChainSpec, String> {
	let wasm_binary = kusama::WASM_BINARY.ok_or("Kusama development wasm not available")?;

	Ok(KusamaChainSpec::from_genesis(
		"Kusama Local Testnet",
		"kusama_local_testnet",
		ChainType::Local,
		move || kusama_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

#[cfg(feature = "westend-native")]
fn westend_local_testnet_genesis(wasm_binary: &[u8]) -> westend::GenesisConfig {
	westend_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Westend local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "westend-native")]
pub fn westend_local_testnet_config() -> Result<WestendChainSpec, String> {
	let wasm_binary = westend::WASM_BINARY.ok_or("Westend development wasm not available")?;

	Ok(WestendChainSpec::from_genesis(
		"Westend Local Testnet",
		"westend_local_testnet",
		ChainType::Local,
		move || westend_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

#[cfg(feature = "rococo-native")]
fn rococo_local_testnet_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice"), get_authority_keys_from_seed("Bob")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Rococo local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "rococo-native")]
pub fn rococo_local_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Rococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Chachacha Local Testnet",
		"chachacha_local_testnet",
		ChainType::Local,
		move || RococoGenesisExt {
			runtime_genesis_config: rococo_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(CHACHACHA_PROTOCOL_ID),
		None,
		Default::default(),
	))
}

/// Wococo is a temporary testnet that uses almost the same runtime as rococo.
#[cfg(feature = "rococo-native")]
fn wococo_local_testnet_genesis(wasm_binary: &[u8]) -> rococo_runtime::GenesisConfig {
	rococo_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Wococo local testnet config (multivalidator Alice + Bob + Charlie + Dave)
#[cfg(feature = "rococo-native")]
pub fn wococo_local_testnet_config() -> Result<RococoChainSpec, String> {
	let wasm_binary = rococo::WASM_BINARY.ok_or("Wococo development wasm not available")?;

	Ok(RococoChainSpec::from_genesis(
		"Wococo Local Testnet",
		"wococo_local_testnet",
		ChainType::Local,
		move || RococoGenesisExt {
			runtime_genesis_config: wococo_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	))
}
