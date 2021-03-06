use num_traits::int::PrimInt;

#[inline(always)]
pub fn is_bitflag_set<T: PrimInt>(source: T, flag: T) -> bool {
    source & flag == flag
}

#[inline(always)]
pub fn set_bitflag<T: PrimInt>(source: &mut T, flag: T) {
    *source = *source | flag
}

#[inline(always)]
pub fn unset_bitflag(source: &mut u8, flag: u8) {
    *source &= !flag
}

#[inline(always)]
pub fn set_bit(source: &mut u8, index: u8) {
    *source |= 1 << index
}

#[inline(always)]
pub fn unset_bit(source: &mut u8, index: u8) {
    *source &= !(1 << index);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        const READ: u8 = 0b00000001;
        const WRITE: u8 = 0b00000010;
        const EXEC: u8 = 0b0000100;
        let mut permissions: u8 = 0b00000000;
        assert!(!is_bitflag_set(permissions, READ) && !is_bitflag_set(permissions, WRITE) && !is_bitflag_set(permissions, EXEC));
        set_bitflag(&mut permissions, READ);
        assert!(is_bitflag_set(permissions, READ) && !is_bitflag_set(permissions, WRITE) && !is_bitflag_set(permissions, EXEC));
        set_bitflag(&mut permissions, WRITE);
        assert!(is_bitflag_set(permissions, READ) && is_bitflag_set(permissions, WRITE) && !is_bitflag_set(permissions, EXEC));
        set_bitflag(&mut permissions, EXEC);
        assert!(is_bitflag_set(permissions, READ) && is_bitflag_set(permissions, WRITE) && is_bitflag_set(permissions, EXEC));
        unset_bitflag(&mut permissions, READ);
        assert!(!is_bitflag_set(permissions, READ) && is_bitflag_set(permissions, WRITE) && is_bitflag_set(permissions, EXEC));
    }
}