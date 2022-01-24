use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;
use std::os::raw::c_char;

use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum SpaceType {
    Constant = 0,
    Processor = 1,
    SpaceBase = 2,
    Internal = 3,
    Fspec = 4,
    Iop = 5,
    Join = 6,
}

impl SpaceType {
    pub fn from_u32(val: u32) -> Option<Self> {
        num::FromPrimitive::from_u32(val)
    }
}

#[derive(Debug, FromPrimitive)]
pub enum Opcode {
    Copy = 1,
    ///< Copy one operand to another
    Load = 2,
    ///< Load from a pointer into a specified address space
    Store = 3,
    ///< Store at a pointer into a specified address space
    Branch = 4,
    ///< Always branch
    CBranch = 5,
    ///< Conditional branch
    BranchInd = 6,
    ///< Indirect branch (jumptable)
    Call = 7,
    ///< Call to an absolute address
    CallInd = 8,
    ///< Call through an indirect address
    CallOther = 9,
    ///< User-defined operation
    Return = 10,
    ///< Return from subroutine
    // Integer/bit operations
    IntEqual = 11,
    ///< Integer comparison, equality (==)
    IntNotEqual = 12,
    ///< Integer comparison, in-equality (!=)
    IntSLess = 13,
    ///< Integer comparison, signed less-than (<)
    IntSLessEqual = 14,
    ///< Integer comparison, signed less-than-or-equal (<=)
    IntLess = 15,
    ///< Integer comparison, unsigned less-than (<)
    // This also indicates a borrow on unsigned substraction
    IntLessEqual = 16,
    ///< Integer comparison, unsigned less-than-or-equal (<=)
    IntZExt = 17,
    ///< Zero extension
    IntSExt = 18,
    ///< Sign extension
    IntAdd = 19,
    ///< Addition, signed or unsigned (+)
    IntSub = 20,
    ///< Subtraction, signed or unsigned (-)
    IntCarry = 21,
    ///< Test for unsigned carry
    IntSCarry = 22,
    ///< Test for signed carry
    IntSBorrow = 23,
    ///< Test for signed borrow
    Int2Comp = 24,
    ///< Twos complement
    IntNegate = 25,
    ///< Logical/bitwise negation (~)
    IntXor = 26,
    ///< Logical/bitwise exclusive-or (^)
    IntAnd = 27,
    ///< Logical/bitwise and (&)
    IntOr = 28,
    ///< Logical/bitwise or (|)
    IntLeft = 29,
    ///< Left shift (<<)
    IntRight = 30,
    ///< Right shift, logical (>>)
    IntSRight = 31,
    ///< Right shift, arithmetic (>>)
    IntMult = 32,
    ///< Integer multiplication, signed and unsigned (*)
    IntDiv = 33,
    ///< Integer division, unsigned (/)
    IntSDiv = 34,
    ///< Integer division, signed (/)
    IntRem = 35,
    ///< Remainder/modulo, unsigned (%)
    IntSRem = 36,
    ///< Remainder/modulo, signed (%)
    BoolNegate = 37,
    ///< Boolean negate (!)
    BoolXor = 38,
    ///< Boolean exclusive-or (^^)
    BoolAnd = 39,
    ///< Boolean and (&&)
    BoolOr = 40,
    ///< Boolean or (||)
    // Floating point operations
    FloatEqual = 41,
    ///< Floating-point comparison, equality (==)
    FloatNotEqual = 42,
    ///< Floating-point comparison, in-equality (!=)
    FloatLess = 43,
    ///< Floating-point comparison, less-than (<)
    FloatLessEqual = 44,
    ///< Floating-point comparison, less-than-or-equal (<=)
    // Slot 45 is currently unused
    FloatNan = 46,
    ///< Not-a-number test (NaN)
    FloatAdd = 47,
    ///< Floating-point addition (+)
    FloatDiv = 48,
    ///< Floating-point division (/)
    FloatMult = 49,
    ///< Floating-point multiplication (*)
    FloatSub = 50,
    ///< Floating-point subtraction (-)
    FloatNeg = 51,
    ///< Floating-point negation (-)
    FloatAbs = 52,
    ///< Floating-point absolute value (abs)
    FloatSqrt = 53,
    ///< Floating-point square root (sqrt)
    FloatInt2Float = 54,
    ///< Convert an integer to a floating-point
    FloatFloat2Float = 55,
    ///< Convert between different floating-point sizes
    FloatTrunc = 56,
    ///< Round towards zero
    FloatCeil = 57,
    ///< Round towards +infinity
    FloatFloor = 58,
    ///< Round towards -infinity
    FloatRound = 59,
    ///< Round towards nearest
    // Internal opcodes for simplification. Not
    // typically generated in a direct translation.

    // Data-flow operations
    MultiEqual = 60,
    ///< Phi-node operator
    Indirect = 61,
    ///< Copy with an indirect effect
    Piece = 62,
    ///< Concatenate
    SubPiece = 63,
    ///< Truncate
    Cast = 64,
    ///< Cast from one data-type to another
    PtrAdd = 65,
    ///< Index into an array ([])
    PtrSub = 66,
    ///< Drill down to a sub-field  (->)
    SegmentOp = 67,
    ///< Look-up a \e segmented address
    CPoolRef = 68,
    ///< Recover a value from the \e constant \e pool
    New = 69,
    ///< Allocate a new object (new)
    Insert = 70,
    ///< Insert a bit-range
    Extract = 71,
    ///< Extract a bit-range
    PopCount = 72,
    ///< Count the 1-bits
    Max = 73,
}

impl Opcode {
    pub fn from_u32(val: u32) -> Option<Self> {
        num::FromPrimitive::from_u32(val)
    }
}

//unsafe impl cxx::ExternType for ffi::spacetype {
//    type Id = type_id!("crate::SpaceType");
//    type Kind = cxx::kind::Trivial;
//}

pub trait AssemblyEmit {
    fn dump(&mut self, addr: &ffi::Address, mnem: &str, body: &str);
}

pub struct RustAssemblyEmit<'a> {
    internal: &'a mut dyn AssemblyEmit,
}

impl<'a> RustAssemblyEmit<'a> {
    pub fn from_internal(internal: &'a mut dyn AssemblyEmit) -> Self {
        Self { internal }
    }

    pub fn dump(&mut self, address: &ffi::Address, mnem: &CxxString, body: &CxxString) {
        let mnem = mnem.to_str().unwrap();
        let body = body.to_str().unwrap();

        self.internal.dump(address, mnem, body);
    }
}

pub trait PCodeEmit {
    /// Callback that will be called when disassembling, emitting the pcode
    /// - address: the address of the machine instruction
    /// - opcode: the opcode of the particular pcode instruction
    /// - outvar: a data about the output varnode
    /// - vars: an array of VarnodeData for each input varnode
    fn dump(
        &mut self,
        address: &ffi::Address,
        opcode: Opcode,
        outvar: Option<&ffi::VarnodeData>,
        vars: &[ffi::VarnodeData],
    );
}

pub struct RustPCodeEmit<'a> {
    pub internal: &'a mut dyn PCodeEmit,
}

pub trait LoadImage {
    fn load_fill(&mut self, ptr: &mut [u8], addr: &ffi::Address);
    fn adjust_vma(&mut self, _adjust: isize) {}
}

pub struct RustLoadImage<'a> {
    internal: &'a mut dyn LoadImage,
}

impl<'a> RustLoadImage<'a> {
    pub fn from_internal(internal: &'a mut dyn LoadImage) -> Self {
        Self { internal }
    }

    unsafe fn load_fill(&mut self, ptr: *mut u8, size: u32, addr: &ffi::Address) {
        let slice = std::slice::from_raw_parts_mut(ptr, size as usize);
        self.internal.load_fill(slice, addr);
    }

    fn adjust_vma(&mut self, adjust: isize) {
        self.internal.adjust_vma(adjust)
    }
}

impl<'a> RustPCodeEmit<'a> {
    pub fn from_internal(internal: &'a mut dyn PCodeEmit) -> Self {
        Self { internal }
    }

    unsafe fn dump(
        &mut self,
        address: &ffi::Address,
        opcode: u32,
        outvar: *const ffi::VarnodeData,
        vars: *const ffi::VarnodeData,
        size: i32,
    ) {
        let outvar = if outvar.is_null() {
            None
        } else {
            Some(&*outvar)
        };
        let vars = std::slice::from_raw_parts(vars, size as usize);
        let opcode = num::FromPrimitive::from_u32(opcode).unwrap();
        self.internal.dump(address, opcode, outvar, vars);
    }
}

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        type RustAssemblyEmit<'a>;
        fn dump(self: &mut RustAssemblyEmit, address: &Address, mnem: &CxxString, body: &CxxString);

        type RustPCodeEmit<'a>;
        unsafe fn dump(
            self: &mut RustPCodeEmit,
            address: &Address,
            opcode: u32,
            outvar: *const VarnodeData,
            vars: *const VarnodeData,
            size: i32,
        );

        type RustLoadImage<'a>;
        unsafe fn load_fill(self: &mut RustLoadImage, ptr: *mut u8, size: u32, addr: &Address);
        //fn get_arch_type(self: &RustLoadImage) -> String;
        fn adjust_vma(self: &mut RustLoadImage, adjust: isize);
    }

    unsafe extern "C++" {
        include!("bridge.hh");

        type Address;
        fn isInvalid(self: &Address) -> bool;
        fn getAddrSize(self: &Address) -> i32;
        fn isBigEndian(self: &Address) -> bool;
        fn getSpace(self: &Address) -> *mut AddrSpace;
        fn getOffset(self: &Address) -> u64;
        fn toPhysical(self: Pin<&mut Address>);
        fn getShortcut(self: &Address) -> c_char;
        fn containedBy(self: &Address, sz: i32, op2: &Address, sz2: i32) -> bool;
        fn justifiedContain(
            self: &Address,
            sz: i32,
            op2: &Address,
            sz2: i32,
            forceleft: bool,
        ) -> i32;
        fn overlap(self: &Address, skip: i32, op: &Address, size: i32) -> i32;
        fn isContiguous(self: &Address, sz: i32, loaddr: &Address, losz: i32) -> bool;
        fn isConstant(self: &Address) -> bool;
        fn renormalize(self: Pin<&mut Address>, size: i32);
        fn isJoin(self: &Address) -> bool;

        type VarnodeData;
        fn getVarnodeDataAddress(data: &VarnodeData) -> UniquePtr<Address>;
        fn getVarnodeSize(data: &VarnodeData) -> u32;

        type spacetype;
        type AddrSpace;
        fn getName(self: &AddrSpace) -> &CxxString;
        //fn getType(self: &AddrSpace) -> spacetype;
        fn getDelay(self: &AddrSpace) -> i32;
        fn getDeadcodeDelay(self: &AddrSpace) -> i32;
        fn getIndex(self: &AddrSpace) -> i32;
        fn getWordSize(self: &AddrSpace) -> u32;
        fn getAddrSize(self: &AddrSpace) -> u32;
        fn getHighest(self: &AddrSpace) -> u64;
        fn getPointerLowerBound(self: &AddrSpace) -> u64;
        fn getPointerUpperBound(self: &AddrSpace) -> u64;
        fn getMinimumPtrSize(self: &AddrSpace) -> i32;
        fn wrapOffset(self: &AddrSpace, off: u64) -> u64;
        fn getShortcut(self: &AddrSpace) -> c_char;
        fn isHeritaged(self: &AddrSpace) -> bool;
        fn doesDeadcode(self: &AddrSpace) -> bool;
        fn hasPhysical(self: &AddrSpace) -> bool;
        fn isBigEndian(self: &AddrSpace) -> bool;
        fn isReverseJustified(self: &AddrSpace) -> bool;
        fn isOverlay(self: &AddrSpace) -> bool;
        fn isOverlayBase(self: &AddrSpace) -> bool;
        fn isOtherSpace(self: &AddrSpace) -> bool;
        fn isTruncated(self: &AddrSpace) -> bool;
        fn hasNearPointers(self: &AddrSpace) -> bool;
        fn numSpacebase(self: &AddrSpace) -> i32;
        fn getSpacebase(self: &AddrSpace, i: i32) -> &VarnodeData;
        fn getSpacebaseFull(self: &AddrSpace, i: i32) -> &VarnodeData;
        fn stackGrowsNegative(self: &AddrSpace) -> bool;
        fn getContain(self: &AddrSpace) -> *mut AddrSpace;

        type OpCode;

        type DocumentStorage;

        type ContextInternal;
        type ContextDatabase;

        fn setVariableDefault(self: Pin<&mut ContextDatabase>, nm: &CxxString, val: u32);
        fn getDefaultValue(self: &ContextDatabase, nm: &CxxString) -> u32;
        fn setVariable(self: Pin<&mut ContextDatabase>, nm: &CxxString, addr: &Address, val: u32);
        fn getVariable(self: &ContextDatabase, nm: &CxxString, addr: &Address) -> u32;

        fn newAddress() -> UniquePtr<Address>;
        fn newContext() -> UniquePtr<ContextDatabase>;
        fn newDocumentStorage(s: &CxxString) -> UniquePtr<DocumentStorage>;

        fn getAddrSpaceType(addr: &AddrSpace) -> u32;

        type Decompiler;
        unsafe fn translate(self: &Decompiler, emit: *mut RustPCodeEmit, addr: u64) -> i32;
        unsafe fn disassemble(self: &Decompiler, emit: *mut RustAssemblyEmit, addr: u64) -> i32;
        unsafe fn getContext(self: Pin<&mut Decompiler>) -> *mut ContextDatabase;
        unsafe fn newDecompiler(
            loadImage: *mut RustLoadImage,
            spec: UniquePtr<DocumentStorage>,
        ) -> UniquePtr<Decompiler>;

    }
}

#[cfg(test)]
mod tests {
    use super::ffi;
    #[test]
    fn test_new() {
        let _a = ffi::newAddress();
        let _a = ffi::newContext();
    }
}
