// implements Display and Serialize
macro_rules! to_str {
    { $type:ty |$s:pat, $f:ident|
        $body:block
    } => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, $f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // `self` can't be used when invoking macros
                let $s = self;
                $body
            }
        }
        impl ::serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(&format!("{}", self))
            }
        }
    };
}

// implements `from_str(str) -> Option` and Deserialize
macro_rules! from_str {
    {$parsable:ty |$arg:ident| $body:block} => {
        impl $parsable {
            pub fn from_str($arg: &str) -> ::std::option::Option<Self>
                $body
        }
        impl<'de> ::serde::Deserialize<'de> for $parsable {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer.deserialize_string($crate::m::Visitor::<$parsable>(::std::marker::PhantomData))
            }
        }
        impl<'de> ::serde::de::Visitor<'de> for $crate::m::Visitor<$parsable> {
            type Value = $parsable;
            fn expecting(&self, _formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // Error formatting is not done here
                Ok(())
            }
            fn visit_str<E>(self, vstr: &str) -> ::std::result::Result<$parsable, E>
            where
                E: ::serde::de::Error,
            {
                match <$parsable>::from_str(vstr) {
                    ::std::option::Option::Some(v) =>
                        Ok(v),
                    ::std::option::Option::None =>
                        Err(E::custom("")),
                }
            }
        }
    };
}