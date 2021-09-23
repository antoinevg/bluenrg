extern crate bluenrg;
extern crate bluetooth_hci as hci;

use bluenrg::event::command::ReturnParameters as BNRGParams;
use bluenrg::event::command::*;
use bluenrg::event::*;
use hci::event::command::ReturnParameters as HciParams;
use hci::event::{Error as HciError, Event as HciEvent, Packet};
use std::time::Duration;

type Event = HciEvent<BlueNRGEvent>;

macro_rules! status_only {
    {
        $($(#[$inner:ident $($args:tt)*])*
        $fn:ident($oc0:expr, $oc1:expr, $return:path);)*
    } => {
        $(
            $(#[$inner $($args)*])*
            #[test]
            fn $fn() {
                let buffer = [0x0E, 4, 8, $oc0, $oc1, 0];
                match Event::new(Packet(&buffer)) {
                    Ok(HciEvent::CommandComplete(event)) => {
                        assert_eq!(event.num_hci_command_packets, 8);
                        match event.return_params {
                            HciParams::Vendor($return(status)) => {
                                assert_eq!(status, hci::Status::Success);
                            }
                            other => panic!("Wrong return parameters: {:?}", other),
                        }
                    }
                    other => panic!("Did not get command complete event: {:?}", other),
                }
            }
        )*
    }
}

status_only! {
    hal_write_config_data(0x0C, 0xFC, BNRGParams::HalWriteConfigData);
    hal_set_tx_power_level(0x0F, 0xFC, BNRGParams::HalSetTxPowerLevel);
    hal_device_standby(0x13, 0xFC, BNRGParams::HalDeviceStandby);
    hal_start_tone(0x15, 0xFC, BNRGParams::HalStartTone);
    hal_stop_tone(0x16, 0xFC, BNRGParams::HalStopTone);

    l2cap_connection_parameter_update_response(
        0x82,
        0xFD,
        BNRGParams::L2CapConnectionParameterUpdateResponse
    );

    gap_set_nondiscoverable(0x81, 0xFC, BNRGParams::GapSetNonDiscoverable);
    gap_set_discoverable(0x83, 0xFC, BNRGParams::GapSetDiscoverable);
    gap_set_direct_connectable(0x84, 0xFC, BNRGParams::GapSetDirectConnectable);
    gap_set_io_capability(0x85, 0xFC, BNRGParams::GapSetIoCapability);
    gap_set_authentication_requirement(0x86, 0xFC, BNRGParams::GapSetAuthenticationRequirement);
    gap_set_authorization_requirement(0x87, 0xFC, BNRGParams::GapSetAuthorizationRequirement);
    gap_pass_key_response(0x88, 0xFC, BNRGParams::GapPassKeyResponse);
    gap_authorization_response(0x89, 0xFC, BNRGParams::GapAuthorizationResponse);
    gap_set_nonconnectable(0x8B, 0xFC, BNRGParams::GapSetNonConnectable);
    gap_set_undirected_connectable(0x8C, 0xFC, BNRGParams::GapSetUndirectedConnectable);
    gap_update_advertising_data(0x8E, 0xFC, BNRGParams::GapUpdateAdvertisingData);
    gap_delete_ad_type(0x8F, 0xFC, BNRGParams::GapDeleteAdType);
    gap_set_event_mask(0x91, 0xFC, BNRGParams::GapSetEventMask);
    gap_configure_white_list(0x92, 0xFC, BNRGParams::GapConfigureWhiteList);
    gap_clear_security_database(0x94, 0xFC, BNRGParams::GapClearSecurityDatabase);
    gap_allow_rebond(0x95, 0xFC, BNRGParams::GapAllowRebond);
    gap_terminate_procedure(0x9D, 0xFC, BNRGParams::GapTerminateProcedure);
    #[cfg(not(feature = "ms"))]
    gap_resolve_private_address(0xA0, 0xFC, BNRGParams::GapResolvePrivateAddress);
    #[cfg(feature = "ms")]
    gap_set_broadcast_mode(0xA1, 0xFC, BNRGParams::GapSetBroadcastMode);
    #[cfg(feature = "ms")]
    gap_start_observation_procedure(0xA2, 0xFC, BNRGParams::GapStartObservationProcedure);
    gap_is_device_bonded(0xA4, 0xFC, BNRGParams::GapIsDeviceBonded);

    gatt_init(0x01, 0xFD, BNRGParams::GattInit);
    gatt_update_characteristic_value(0x06, 0xFD, BNRGParams::GattUpdateCharacteristicValue);
    gatt_delete_characteristic(0x07, 0xFD, BNRGParams::GattDeleteCharacteristic);
    gatt_delete_service(0x08, 0xFD, BNRGParams::GattDeleteService);
    gatt_delete_included_service(0x09, 0xFD, BNRGParams::GattDeleteIncludedService);
    gatt_set_event_mask(0x0A, 0xFD, BNRGParams::GattSetEventMask);
    gatt_write_without_response(0x23, 0xFD, BNRGParams::GattWriteWithoutResponse);
    gatt_signed_write_without_response(0x24, 0xFD, BNRGParams::GattSignedWriteWithoutResponse);
    gatt_confirm_indication(0x25, 0xFD, BNRGParams::GattConfirmIndication);
    gatt_write_response(0x26, 0xFD, BNRGParams::GattWriteResponse);
    gatt_allow_read(0x27, 0xFD, BNRGParams::GattAllowRead);
    gatt_set_security_permission(0x28, 0xFD, BNRGParams::GattSetSecurityPermission);
    gatt_set_descriptor_value(0x29, 0xFD, BNRGParams::GattSetDescriptorValue);
    #[cfg(feature = "ms")]
    gatt_update_long_characteristic_value(
        0x2C,
        0xFD,
        BNRGParams::GattUpdateLongCharacteristicValue
    );
}

#[test]
fn hal_write_config_data_nonstandard_status() {
    for (code, expected_status) in [
        (0x41, Status::Failed),
        (0x42, Status::InvalidParameters),
        (0x46, Status::NotAllowed),
        (0x47, Status::Error),
        (0x48, Status::AddressNotResolved),
        (0x49, Status::FlashReadFailed),
        (0x4A, Status::FlashWriteFailed),
        (0x4B, Status::FlashEraseFailed),
        (0x50, Status::InvalidCid),
        (0x54, Status::TimerNotValidLayer),
        (0x55, Status::TimerInsufficientResources),
        (0x5A, Status::CsrkNotFound),
        (0x5B, Status::IrkNotFound),
        (0x5C, Status::DeviceNotFoundInDatabase),
        (0x5D, Status::SecurityDatabaseFull),
        (0x5E, Status::DeviceNotBonded),
        (0x5F, Status::DeviceInBlacklist),
        (0x60, Status::InvalidHandle),
        (0x61, Status::InvalidParameter),
        (0x62, Status::OutOfHandle),
        (0x63, Status::InvalidOperation),
        (0x64, Status::InsufficientResources),
        (0x65, Status::InsufficientEncryptionKeySize),
        (0x66, Status::CharacteristicAlreadyExists),
        (0x82, Status::NoValidSlot),
        (0x83, Status::ScanWindowTooShort),
        (0x84, Status::NewIntervalFailed),
        (0x85, Status::IntervalTooLarge),
        (0x86, Status::LengthFailed),
        (0xFF, Status::Timeout),
        (0xF0, Status::ProfileAlreadyInitialized),
        (0xF1, Status::NullParameter),
    ]
    .iter()
    {
        let buffer = [0x0E, 4, 8, 0x0C, 0xFC, *code];
        match Event::new(Packet(&buffer)) {
            Ok(HciEvent::CommandComplete(event)) => {
                assert_eq!(event.num_hci_command_packets, 8);
                match event.return_params {
                    HciParams::Vendor(BNRGParams::HalWriteConfigData(status)) => {
                        assert_eq!(status, hci::Status::Vendor(*expected_status));
                    }
                    other => panic!("Wrong return parameters: {:?}", other),
                }
            }
            other => panic!("Did not get command complete event: {:?}", other),
        }
    }
}

#[test]
fn hal_read_config_data_public_addr() {
    let buffer = [0x0E, 10, 8, 0x0D, 0xFC, 0, 1, 2, 3, 4, 5, 6];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalReadConfigData(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.value,
                        HalConfigParameter::PublicAddress(hci::BdAddr([1, 2, 3, 4, 5, 6]))
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_read_config_data_diversifier() {
    let buffer = [0x0E, 6, 8, 0x0D, 0xFC, 0, 1, 2];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalReadConfigData(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.value, HalConfigParameter::Diversifier(0x0201));
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_read_config_data_key() {
    let buffer = [
        0x0E, 20, 8, 0x0D, 0xFC, 0, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB,
        0xC, 0xD, 0xE, 0xF,
    ];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalReadConfigData(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.value,
                        HalConfigParameter::EncryptionKey(hci::host::EncryptionKey([
                            0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD,
                            0xE, 0xF
                        ]))
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_read_config_byte() {
    let buffer = [0x0E, 5, 8, 0x0D, 0xFC, 0, 0];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalReadConfigData(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.value, HalConfigParameter::Byte(0));
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_read_config_invalid() {
    let buffer = [0x0E, 7, 8, 0x0D, 0xFC, 0, 0, 1, 2];
    match Event::new(Packet(&buffer)) {
        Err(HciError::Vendor(BlueNRGError::BadConfigParameterLength(len))) => {
            assert_eq!(len, 3);
        }
        other => panic!("Did not get bad parameter length: {:?}", other),
    }
}

#[test]
fn hal_get_tx_test_packet_count() {
    let buffer = [0x0E, 8, 8, 0x14, 0xFC, 0, 0x1, 0x2, 0x3, 0x4];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalGetTxTestPacketCount(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.packet_count, 0x04030201);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_get_link_status() {
    let buffer = [
        0x0E, 28, 8, 0x17, 0xFC, 0, 0, 1, 2, 3, 4, 5, 6, 7, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
        0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalGetLinkStatus(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.clients,
                        [
                            ClientStatus {
                                state: LinkState::Idle,
                                conn_handle: hci::ConnectionHandle(0x0100),
                            },
                            ClientStatus {
                                state: LinkState::Advertising,
                                conn_handle: hci::ConnectionHandle(0x0302),
                            },
                            ClientStatus {
                                state: LinkState::ConnectedAsPeripheral,
                                conn_handle: hci::ConnectionHandle(0x0504),
                            },
                            ClientStatus {
                                state: LinkState::Scanning,
                                conn_handle: hci::ConnectionHandle(0x0706),
                            },
                            ClientStatus {
                                state: LinkState::Reserved,
                                conn_handle: hci::ConnectionHandle(0x0908),
                            },
                            ClientStatus {
                                state: LinkState::ConnectedAsPrimary,
                                conn_handle: hci::ConnectionHandle(0x0B0A),
                            },
                            ClientStatus {
                                state: LinkState::TxTest,
                                conn_handle: hci::ConnectionHandle(0x0D0C),
                            },
                            ClientStatus {
                                state: LinkState::RxTest,
                                conn_handle: hci::ConnectionHandle(0x0F0E),
                            },
                        ]
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_get_link_status_invalid_status() {
    let buffer = [
        0x0E, 28, 8, 0x17, 0xFC, 0, 8, 1, 2, 3, 4, 5, 6, 7, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
        0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ];
    match Event::new(Packet(&buffer)) {
        Err(HciError::Vendor(BlueNRGError::UnknownLinkState(value))) => {
            assert_eq!(value, 8);
        }
        other => panic!("Did not get unknown link state: {:?}", other),
    }
}

#[test]
fn hal_get_firmware_revision() {
    let buffer = [0x0E, 6, 8, 0x00, 0xFC, 0, 1, 2];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalGetFirmwareRevision(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.revision, 0x0201);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn hal_get_anchor_period() {
    let buffer = [
        0x0E, 12, 8, 0x19, 0xFC, 0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8,
    ];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::HalGetAnchorPeriod(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.anchor_interval,
                        Duration::from_micros(625 * 0x04030201)
                    );
                    assert_eq!(params.max_slot, Duration::from_micros(625 * 0x08070605));
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gap_init() {
    let buffer = [
        0x0E, 10, 8, 0x8A, 0xFC, 0, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
    ];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapInit(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.service_handle, bluenrg::gatt::ServiceHandle(0x0201));
                    assert_eq!(
                        params.dev_name_handle,
                        bluenrg::gatt::CharacteristicHandle(0x0403)
                    );
                    assert_eq!(
                        params.appearance_handle,
                        bluenrg::gatt::CharacteristicHandle(0x0605)
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gap_get_security_level() {
    let buffer = [0x0E, 8, 1, 0x90, 0xFC, 0, 0, 1, 0, 2];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapGetSecurityLevel(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.mitm_protection_required, false);
                    assert_eq!(params.bonding_required, true);
                    assert_eq!(params.out_of_band_data_present, false);
                    assert_eq!(params.pass_key_required, PassKeyRequirement::Generated);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gap_get_security_level_bad_bool() {
    let buffer = [0x0E, 8, 1, 0x90, 0xFC, 0, 2, 1, 0, 2];
    match Event::new(Packet(&buffer)) {
        Err(HciError::Vendor(BlueNRGError::BadBooleanValue(value))) => {
            assert_eq!(value, 2);
        }
        other => panic!("Did not get bad boolean: {:?}", other),
    }
}

#[test]
fn gap_get_security_level_bad_pass_key_requirement() {
    let buffer = [0x0E, 8, 1, 0x90, 0xFC, 0, 0, 1, 0, 3];
    match Event::new(Packet(&buffer)) {
        Err(HciError::Vendor(BlueNRGError::BadPassKeyRequirement(value))) => {
            assert_eq!(value, 3);
        }
        other => panic!("Did not get bad pass key requirement: {:?}", other),
    }
}

#[cfg(feature = "ms")]
#[test]
fn gap_resolve_private_address() {
    let buffer = [0x0E, 10, 1, 0xA0, 0xFC, 0, 1, 2, 3, 4, 5, 6];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapResolvePrivateAddress(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.bd_addr, Some(hci::BdAddr([1, 2, 3, 4, 5, 6])));
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[cfg(feature = "ms")]
#[test]
fn gap_resolve_private_address_failed() {
    let buffer = [0x0E, 4, 1, 0xA0, 0xFC, 0x12];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapResolvePrivateAddress(params)) => {
                    assert_eq!(params.status, hci::Status::InvalidParameters,);
                    assert_eq!(params.bd_addr, None);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[cfg(feature = "ms")]
#[test]
fn gap_resolve_private_address_failed_mixed_signals() {
    let buffer = [0x0E, 10, 1, 0xA0, 0xFC, 0x12, 1, 2, 3, 4, 5, 6];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapResolvePrivateAddress(params)) => {
                    assert_eq!(params.status, hci::Status::InvalidParameters,);
                    assert_eq!(params.bd_addr, None);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gap_get_bonded_addresses() {
    let buffer = [
        0x0E, 19, 1, 0xA3, 0xFC, 0, 2, 0, 1, 2, 3, 4, 5, 6, 1, 6, 5, 4, 3, 2, 1,
    ];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapGetBondedDevices(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.bonded_addresses(),
                        [
                            hci::BdAddrType::Public(hci::BdAddr([1, 2, 3, 4, 5, 6])),
                            hci::BdAddrType::Random(hci::BdAddr([6, 5, 4, 3, 2, 1])),
                        ]
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gap_get_bonded_addresses_partial() {
    let buffer = [
        0x0E, 18, 1, 0xA3, 0xFC, 0, 2, 0, 1, 2, 3, 4, 5, 6, 1, 6, 5, 4, 3, 2,
    ];
    match Event::new(Packet(&buffer)) {
        Err(HciError::Vendor(BlueNRGError::PartialBondedDeviceAddress)) => (),
        other => panic!("Did not get partial bonded device address: {:?}", other),
    }
}

#[test]
fn gap_get_bonded_addresses_bad_addr_type() {
    let buffer = [
        0x0E, 19, 1, 0xA3, 0xFC, 0, 2, 2, 1, 2, 3, 4, 5, 6, 1, 6, 5, 4, 3, 2, 1,
    ];
    match Event::new(Packet(&buffer)) {
        Err(HciError::Vendor(BlueNRGError::BadBdAddrType(2))) => (),
        other => panic!("Did not get bad address type: {:?}", other),
    }
}

#[test]
fn gap_get_bonded_addresses_failed() {
    let buffer = [0x0E, 4, 1, 0xA3, 0xFC, 0x12];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapGetBondedDevices(params)) => {
                    assert_eq!(params.status, hci::Status::InvalidParameters,);
                    assert_eq!(params.bonded_addresses(), []);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gap_get_bonded_addresses_failed_mixed_signals() {
    let buffer = [0x0E, 12, 1, 0xA3, 0xFC, 0x12, 1, 0, 1, 2, 3, 4, 5, 6];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GapGetBondedDevices(params)) => {
                    assert_eq!(params.status, hci::Status::InvalidParameters,);
                    assert_eq!(params.bonded_addresses(), []);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gatt_add_service() {
    let buffer = [0x0E, 6, 1, 0x02, 0xFD, 0x00, 0x01, 0x02];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GattAddService(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.service_handle, bluenrg::gatt::ServiceHandle(0x0201));
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gatt_include_service() {
    let buffer = [0x0E, 6, 1, 0x03, 0xFD, 0x00, 0x01, 0x02];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GattIncludeService(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.service_handle, bluenrg::gatt::ServiceHandle(0x0201));
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gatt_add_characteristic() {
    let buffer = [0x0E, 6, 1, 0x04, 0xFD, 0x00, 0x01, 0x02];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GattAddCharacteristic(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.characteristic_handle,
                        bluenrg::gatt::CharacteristicHandle(0x0201)
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gatt_add_characteristic_descriptor() {
    let buffer = [0x0E, 6, 1, 0x05, 0xFD, 0x00, 0x01, 0x02];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GattAddCharacteristicDescriptor(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(
                        params.descriptor_handle,
                        bluenrg::gatt::DescriptorHandle(0x0201)
                    );
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[test]
fn gatt_read_handle_value() {
    let buffer = [0x0E, 9, 1, 0x2A, 0xFD, 0x00, 0x03, 0x00, 1, 2, 3];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GattReadHandleValue(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.value(), &[1, 2, 3]);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}

#[cfg(feature = "ms")]
#[test]
fn gatt_read_handle_value_offset() {
    let buffer = [0x0E, 9, 1, 0x2B, 0xFD, 0x00, 0x03, 0x00, 1, 2, 3];
    match Event::new(Packet(&buffer)) {
        Ok(HciEvent::CommandComplete(event)) => {
            assert_eq!(event.num_hci_command_packets, 1);
            match event.return_params {
                HciParams::Vendor(BNRGParams::GattReadHandleValueOffset(params)) => {
                    assert_eq!(params.status, hci::Status::Success);
                    assert_eq!(params.value(), &[1, 2, 3]);
                }
                other => panic!("Wrong return parameters: {:?}", other),
            }
        }
        other => panic!("Did not get command complete event: {:?}", other),
    }
}
