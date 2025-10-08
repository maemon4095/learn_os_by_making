use core::{marker::PhantomData, mem::size_of};

// REF: https://uefi.org/specs/UEFI/2.10/02_Overview.html#data-types
extern "C" {
    pub type U16Str;
}

impl U16Str {
    pub unsafe fn from_raw_parts(ptr: *const u16) -> *const Self {
        ptr as *const Self
    }

    pub fn code_units(&self) -> CodeUnits<'_> {
        CodeUnits {
            _phantom: PhantomData,
            ptr: { (self as *const Self) as *const u16 },
        }
    }
}

// reference of U16Str must be thin pointer
const _: () = assert!(size_of::<&U16Str>() == size_of::<&()>());
const _: () = assert!(size_of::<*const U16Str>() == size_of::<*const ()>());

pub struct CodeUnits<'a> {
    _phantom: PhantomData<&'a ()>,
    ptr: *const u16,
}

impl<'a> Iterator for CodeUnits<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let c = *self.ptr;
            if c == 0 {
                None
            } else {
                self.ptr = self.ptr.add(1);
                Some(c)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test() {
    //     let text = "Lorem ipsum dolor sit amet sit sed sit in et clita.";
    //     let utf16bin = [
    //         76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32,
    //         115, 105, 116, 32, 97, 109, 101, 116, 32, 115, 105, 116, 32, 115, 101, 100, 32, 115,
    //         105, 116, 32, 105, 110, 32, 101, 116, 32, 99, 108, 105, 116, 97, 46,
    //     ];
    //     let ptr = utf16bin.as_ptr();
    //     let u16str = unsafe { &*U16Str::from_raw_parts(ptr) };
    //     let units = u16str.code_units();
    //     assert!(utf16bin.iter().copied().eq(units))
    // }
}
