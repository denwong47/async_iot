use std::marker::PhantomData;

use systemstat::{self, Platform};

use crate::{results, traits::ResultToOption};

use super::{cpu::cpu_load, networks::networks, temperatures::temperatures};

/// An empty struct that acts as a wrapper for associated functions.
pub struct SystemState {
    /// Prevents instantiation
    _private: PhantomData<()>,
}

macro_rules! expand_fields {
    (
        $((
            $field: ident,
            $func: expr
        )),*$(,)?
    ) => {
        impl SystemState {
            pub fn get(
                keys: &[&str],
            ) -> results::ResultJson {
                let mut json = results::ResultJson::new();
                let sys = systemstat::System::new();

                $(
                    if keys.contains(&stringify!($field)) {
                        let result = $func(&sys);
                        json.append_result(
                            &stringify!($field),
                            results::ResultState::from(&result),
                            result.to_option(),
                        );
                    }
                )*

                json
            }

            /// Get a [`results::ResultJson`] with all the available
            /// keys.
            pub fn all() -> results::ResultJson {
                Self::get(
                    &[
                        $(
                            stringify!($field),
                        )*
                    ]
                )
            }
        }
    }
}

expand_fields!(
    (cpu_load, cpu_load),
    (temperatures, temperatures),
    (networks, networks),
);
