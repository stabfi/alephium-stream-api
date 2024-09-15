use hex::FromHexError;

enum AddressType {
    P2PKH = 0x00,
    P2MPKH = 0x01,
    P2SH = 0x02,
    P2C = 0x03,
}

pub fn address_from_contract_id(contract_id: &'_ str) -> Result<String, FromHexError> {
    let mut buffer = [0u8; 33];

    buffer[0] = AddressType::P2C as u8;
    hex::decode_to_slice(contract_id, &mut buffer[1..])?;

    Ok(bs58::encode(buffer).into_string())
}
