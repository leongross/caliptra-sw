// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with caliptra-rtl repo at e181dafe3035dc2d4e16dd2ecbfd0d1565a63525
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
/// A zero-sized type that represents ownership of this
/// peripheral, used to get access to a Register lock. Most
/// programs create one of these in unsafe code near the top of
/// main(), and pass it to the driver responsible for managing
/// all access to the hardware.
pub struct KvReg {
    _priv: (),
}
impl KvReg {
    pub const PTR: *mut u32 = 0x10018000 as *mut u32;
    /// # Safety
    ///
    /// Caller must ensure that all concurrent use of this
    /// peripheral in the firmware is done so in a compatible
    /// way. The simplest way to enforce this is to only call
    /// this function once.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self { _priv: () }
    }
    /// Returns a register block that can be used to read
    /// registers from this peripheral, but cannot write.
    #[inline(always)]
    pub fn regs(&self) -> RegisterBlock<ureg::RealMmio> {
        RegisterBlock {
            ptr: Self::PTR,
            mmio: core::default::Default::default(),
        }
    }
    /// Return a register block that can be used to read and
    /// write this peripheral's registers.
    #[inline(always)]
    pub fn regs_mut(&mut self) -> RegisterBlock<ureg::RealMmioMut> {
        RegisterBlock {
            ptr: Self::PTR,
            mmio: core::default::Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
pub struct RegisterBlock<TMmio: ureg::Mmio + core::borrow::Borrow<TMmio>> {
    ptr: *mut u32,
    mmio: TMmio,
}
impl<TMmio: ureg::Mmio + core::default::Default> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self {
            ptr,
            mmio: core::default::Default::default(),
        }
    }
}
impl<TMmio: ureg::Mmio> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    #[inline(always)]
    pub unsafe fn new_with_mmio(ptr: *mut u32, mmio: TMmio) -> Self {
        Self { ptr, mmio }
    }
    /// Controls for each keyvault and pcr entry
    ///
    /// Read value: [`kv::regs::KvctrlReadVal`]; Write value: [`kv::regs::KvctrlWriteVal`]
    #[inline(always)]
    pub fn key_ctrl(&self) -> ureg::Array<32, ureg::RegRef<crate::kv::meta::KeyCtrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Key Entries are not readable or writeable by software
    ///
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn key_entry(
        &self,
    ) -> ureg::Array<32, ureg::Array<12, ureg::RegRef<crate::kv::meta::KeyEntry, &TMmio>>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x600 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`kv::regs::ClearSecretsReadVal`]; Write value: [`kv::regs::ClearSecretsWriteVal`]
    #[inline(always)]
    pub fn clear_secrets(&self) -> ureg::RegRef<crate::kv::meta::ClearSecrets, &TMmio> {
        unsafe {
            ureg::RegRef::new_with_mmio(
                self.ptr.wrapping_add(0xc00 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct ClearSecretsReadVal(u32);
    impl ClearSecretsReadVal {
        /// Fill the keyvault with debug values
        #[inline(always)]
        pub fn wr_debug_values(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Selects between debug value 0 or 1 parameter to write to keyvault
        #[inline(always)]
        pub fn sel_debug_value(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> ClearSecretsWriteVal {
            ClearSecretsWriteVal(self.0)
        }
    }
    impl From<u32> for ClearSecretsReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ClearSecretsReadVal> for u32 {
        #[inline(always)]
        fn from(val: ClearSecretsReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct ClearSecretsWriteVal(u32);
    impl ClearSecretsWriteVal {
        /// Fill the keyvault with debug values
        #[inline(always)]
        pub fn wr_debug_values(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Selects between debug value 0 or 1 parameter to write to keyvault
        #[inline(always)]
        pub fn sel_debug_value(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
    }
    impl From<u32> for ClearSecretsWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<ClearSecretsWriteVal> for u32 {
        #[inline(always)]
        fn from(val: ClearSecretsWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KvctrlReadVal(u32);
    impl KvctrlReadVal {
        /// Lock writes to this entry. Writes will be suppressed and an error will be recorded.
        #[inline(always)]
        pub fn lock_wr(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Lock use of this entry. Reads will be suppressed and an error will be recorded.
        #[inline(always)]
        pub fn lock_use(&self) -> bool {
            ((self.0 >> 1) & 1) != 0
        }
        /// Clear the data stored in this entry. Lock write will prevent this clear.
        #[inline(always)]
        pub fn clear(&self) -> bool {
            ((self.0 >> 2) & 1) != 0
        }
        /// Reserved
        #[inline(always)]
        pub fn rsvd0(&self) -> bool {
            ((self.0 >> 3) & 1) != 0
        }
        /// Reserved
        #[inline(always)]
        pub fn rsvd1(&self) -> u32 {
            (self.0 >> 4) & 0x1f
        }
        /// Destination valid bits stored as an array for ease of use in RTL.
        /// [br]dest_valid[0] = hmac_key_dest_valid
        /// [br]dest_valid[1] = hmac_block_dest_valid
        /// [br]dest_valid[2] = sha_block_dest_valid
        /// [br]dest_valid[3] = ecc_pkey_dest_valid
        /// [br]dest_valid[4] = ecc_seed_dest_valid
        /// [br]dest_valid[5] = rsvd
        /// [br]dest_valid[6] = rsvd
        /// [br]dest_valid[7] = rsvd
        #[inline(always)]
        pub fn dest_valid(&self) -> u32 {
            (self.0 >> 9) & 0xff
        }
        /// Stores the offset of the last valid dword, used to indicate last cycle on reads.
        #[inline(always)]
        pub fn last_dword(&self) -> u32 {
            (self.0 >> 17) & 0xf
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> KvctrlWriteVal {
            KvctrlWriteVal(self.0)
        }
    }
    impl From<u32> for KvctrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KvctrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: KvctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct KvctrlWriteVal(u32);
    impl KvctrlWriteVal {
        /// Lock writes to this entry. Writes will be suppressed and an error will be recorded.
        #[inline(always)]
        pub fn lock_wr(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
        /// Lock use of this entry. Reads will be suppressed and an error will be recorded.
        #[inline(always)]
        pub fn lock_use(self, val: bool) -> Self {
            Self((self.0 & !(1 << 1)) | (u32::from(val) << 1))
        }
        /// Clear the data stored in this entry. Lock write will prevent this clear.
        #[inline(always)]
        pub fn clear(self, val: bool) -> Self {
            Self((self.0 & !(1 << 2)) | (u32::from(val) << 2))
        }
        /// Reserved
        #[inline(always)]
        pub fn rsvd0(self, val: bool) -> Self {
            Self((self.0 & !(1 << 3)) | (u32::from(val) << 3))
        }
        /// Reserved
        #[inline(always)]
        pub fn rsvd1(self, val: u32) -> Self {
            Self((self.0 & !(0x1f << 4)) | ((val & 0x1f) << 4))
        }
    }
    impl From<u32> for KvctrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<KvctrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: KvctrlWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    pub mod selector {}
}
pub mod meta {
    //! Additional metadata needed by ureg.
    pub type KeyCtrl =
        ureg::ReadWriteReg32<0, crate::kv::regs::KvctrlReadVal, crate::kv::regs::KvctrlWriteVal>;
    pub type KeyEntry = ureg::WriteOnlyReg32<0, u32>;
    pub type ClearSecrets = ureg::ReadWriteReg32<
        0,
        crate::kv::regs::ClearSecretsReadVal,
        crate::kv::regs::ClearSecretsWriteVal,
    >;
}
