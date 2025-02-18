pub static VALID_PROOF: [u8; include_bytes!("resources/bin/compressed_snark.bin").len()] =
    *include_bytes!("resources/bin/compressed_snark.bin");
pub static VALID_VK: [u8; include_bytes!("resources/bin/vk.bin").len()] =
    *include_bytes!("resources/bin/vk.bin");
pub static VALID_PUBS: [u8; include_bytes!("resources/bin/pubs.bin").len()] =
    *include_bytes!("resources/bin/pubs.bin");
