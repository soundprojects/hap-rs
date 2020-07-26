// this file is auto-generated by build.rs

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		active::ActiveCharacteristic,
		active_identifier::ActiveIdentifierCharacteristic,
		configured_name::ConfiguredNameCharacteristic,
		sleep_discovery_mode::SleepDiscoveryModeCharacteristic,
		brightness::BrightnessCharacteristic,
		closed_captions::ClosedCaptionsCharacteristic,
		display_order::DisplayOrderCharacteristic,
		current_media_state::CurrentMediaStateCharacteristic,
		target_media_state::TargetMediaStateCharacteristic,
		picture_mode::PictureModeCharacteristic,
		power_mode_selection::PowerModeSelectionCharacteristic,
		remote_key::RemoteKeyCharacteristic,
	},
    HapType,
};

/// Television Service.
#[derive(Debug, Default)]
pub struct TelevisionService {
    /// ID of the Television Service.
    id: u64,
    /// `HapType` of the Television Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic (required).
	pub active: ActiveCharacteristic,
	/// Active Identifier Characteristic (required).
	pub active_identifier: ActiveIdentifierCharacteristic,
	/// Configured Name Characteristic (required).
	pub configured_name: ConfiguredNameCharacteristic,
	/// Sleep Discovery Mode Characteristic (required).
	pub sleep_discovery_mode: SleepDiscoveryModeCharacteristic,

	/// Brightness Characteristic (optional).
	pub brightness: Option<BrightnessCharacteristic>,
	/// Closed Captions Characteristic (optional).
	pub closed_captions: Option<ClosedCaptionsCharacteristic>,
	/// Display Order Characteristic (optional).
	pub display_order: Option<DisplayOrderCharacteristic>,
	/// Current Media State Characteristic (optional).
	pub current_media_state: Option<CurrentMediaStateCharacteristic>,
	/// Target Media State Characteristic (optional).
	pub target_media_state: Option<TargetMediaStateCharacteristic>,
	/// Picture Mode Characteristic (optional).
	pub picture_mode: Option<PictureModeCharacteristic>,
	/// Power Mode Selection Characteristic (optional).
	pub power_mode_selection: Option<PowerModeSelectionCharacteristic>,
	/// Remote Key Characteristic (optional).
	pub remote_key: Option<RemoteKeyCharacteristic>,
}

impl TelevisionService {
    /// Creates a new Television Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Television,
			active: ActiveCharacteristic::new(id + 1 + 0, accessory_id),
			active_identifier: ActiveIdentifierCharacteristic::new(id + 1 + 1, accessory_id),
			configured_name: ConfiguredNameCharacteristic::new(id + 1 + 2, accessory_id),
			sleep_discovery_mode: SleepDiscoveryModeCharacteristic::new(id + 1 + 3, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for TelevisionService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.active,
			&self.active_identifier,
			&self.configured_name,
			&self.sleep_discovery_mode,
		];
		if let Some(c) = &self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &self.closed_captions {
		    characteristics.push(c);
		}
		if let Some(c) = &self.display_order {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_media_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_media_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.picture_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &self.power_mode_selection {
		    characteristics.push(c);
		}
		if let Some(c) = &self.remote_key {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
			&mut self.active_identifier,
			&mut self.configured_name,
			&mut self.sleep_discovery_mode,
		];
		if let Some(c) = &mut self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.closed_captions {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.display_order {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_media_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_media_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.picture_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.power_mode_selection {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.remote_key {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for TelevisionService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}