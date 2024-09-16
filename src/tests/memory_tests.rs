use crate::memory::Memory;

#[test]
fn test_write_read_u8() {
    let mut mem = Memory::<256>::new();
    mem.write_mem_u8(0, 42).unwrap();
    assert_eq!(mem.read_mem_u8(0), Ok(42));
}

#[test]
fn test_write_read_u16() {
    let mut mem = Memory::<256>::new();
    mem.write_mem_u8(0, 1).unwrap();
    mem.write_mem_u8(1, 2).unwrap();
    assert_eq!(mem.read_mem_u16(0), Ok(258));
}