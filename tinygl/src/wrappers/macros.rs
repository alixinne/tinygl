macro_rules! impl_name {
    (pub $t:ty) => {
        #[cfg(not(target_arch = "wasm32"))]
        pub fn name(&self) -> $t {
            self.name
        }

        #[cfg(target_arch = "wasm32")]
        pub fn name(&self) -> $t {
            &self.name
        }
    };

    ($t:ty) => {
        #[cfg(not(target_arch = "wasm32"))]
        fn name(&self) -> $t {
            self.name
        }

        #[cfg(target_arch = "wasm32")]
        fn name(&self) -> $t {
            &self.name
        }
    };
}

macro_rules! impl_nnew {
    ($e:ident, $fd:ident, $fw:ident) => {
        #[cfg(not(target_arch = "wasm32"))]
        pub fn new(gl: &crate::Context) -> crate::Result<Self> {
            unsafe {
                let mut name = 0u32;
                gl.$fd(1, &mut name);
                if name == 0 {
                    return Err(crate::Error::$e(OpenGlErrorCode(gl.get_error())));
                }

                Ok(Self { name })
            }
        }

        #[cfg(target_arch = "wasm32")]
        pub fn new(gl: &crate::Context) -> crate::Result<Self> {
            Ok(gl
                .$fw()
                .map(|name| Self { name })
                .ok_or_else(|| crate::Error::$e(OpenGlErrorCode(gl.get_error())))?)
        }
    };
}

macro_rules! impl_ndrop {
    ($t:ty, $fd:ident, $fw:ident) => {
        impl super::GlDrop for $t {
            #[cfg(not(target_arch = "wasm32"))]
            unsafe fn drop(&mut self, gl: &crate::Context) {
                gl.$fd(1, &self.name);
            }

            #[cfg(target_arch = "wasm32")]
            unsafe fn drop(&mut self, gl: &crate::Context) {
                gl.$fw(Some(&self.name));
            }
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! make_name {
    (Option => $e:expr) => {
        $e
    };

    ($e:expr) => {
        $e
    };
}

#[cfg(target_arch = "wasm32")]
macro_rules! make_name {
    (Option => $e:expr) => {
        Some(&$e)
    };

    ($e:expr) => {
        &$e
    };
}
