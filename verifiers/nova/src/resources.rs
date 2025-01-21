// pub static VALID_VK: sp_core::H256 = sp_core::H256(hex_literal::hex!(
//     "0000000000000000000000000000000000000000000000000000000000000001"
// ));

// pub static VALID_PROOF: [u8; 512] = hex_literal::hex!("00...02");

// pub static VALID_PUBS: [u8; 32] =
//     hex_literal::hex!("0000000000000000000000000000000000000000000000000000000000000003");

pub static VALID_PROOF: [u8; include_bytes!("resources/bin/compressed_snark.bin").len()] =
    *include_bytes!("resources/bin/compressed_snark.bin");
pub static VALID_VK: [u8; include_bytes!("resources/bin/vk.bin").len()] =
    *include_bytes!("resources/bin/vk.bin");

pub static VALID_PUBS: [u8; include_bytes!("resources/bin/pubs.bin").len()] =
    *include_bytes!("resources/bin/pubs.bin");
