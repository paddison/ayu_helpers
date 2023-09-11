pub(crate) mod generate;
pub(crate) mod custom;
pub(crate) mod file;

const DUMMY_MEMADDR: u64 = 0xffee0000;

fn generate_mem_address_from_id(id: u64) -> u64 {
    DUMMY_MEMADDR | id
}