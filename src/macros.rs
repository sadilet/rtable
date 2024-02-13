macro_rules! convert {
    (enum $enum:ty, variant $var:path: type $ty:ty) => {
        impl<'a> From<$ty> for $enum {
            fn from(value: $ty) -> $enum {
                $var(value)
            }
        }

        impl<'a> crate::util::convert::TryAsRef<$ty> for $enum {
            fn try_as_ref(&self) -> Option<&$ty> {
                if let $var(value) = self {
                    Some(value)
                } else {
                    None
                }
            }
        }

        impl<'a> crate::util::convert::TryAsMut<$ty> for $enum {
            fn try_as_mut(&mut self) -> Option<&mut $ty> {
                if let $var(value) = self {
                    Some(value)
                } else {
                    None
                }
            }
        }
    };

    (enum $enum:ty, type $into:ty: from $($from:ty),*) => {
        $(impl<'a> From<$from> for $enum {
            fn from(value: $from) -> $enum {
                <$enum>::from(<$into>::from(value))
            }
        })*
    };

    (enum $enum:ty, type $val:ty: ref $ref:ty) => {
        impl<'a> crate::util::convert::TryAsRef<$ref> for $enum {
            fn try_as_ref(&self) -> Option<&$ref> {
                crate::util::convert::TryAsRef::<$val>::try_as_ref(self)
                    .map(AsRef::as_ref)
            }
        }

        impl<'a> crate::util::convert::TryAsMut<$ref> for $enum {
            fn try_as_mut(&mut self) -> Option<&mut $ref> {
                crate::util::convert::TryAsMut::<$val>::try_as_mut(self)
                    .map(AsMut::as_mut)
            }
        }
    };

    (enum $enum:ty, type $into:ty: from_as $($from:ty),*) => {
        $(impl<'a> From<$from> for $enum {
            fn from(value: $from) -> $enum {
                <$enum>::from(value as $into)
            }
        })*
    };

    (enum $enum:ty: cow $r:ty, $t:ty) => {
        impl<'a> From<&'a $r> for $enum {
            fn from(value: &'a $r) -> $enum {
                <$enum>::from(Cow::Borrowed(value))
            }
        }

        impl<'a> From<$t> for $enum {
            fn from(value: $t) -> $enum {
                <$enum as From<Cow<'a, $r>>>::from(Cow::Owned(value))
            }
        }

        impl<'a> crate::util::convert::TryAsRef<$r> for $enum {
            fn try_as_ref(&self) -> Option<&$r> {
                crate::util::convert::TryAsRef::<::std::borrow::Cow<'a, $r>>
                    ::try_as_ref(self)
                    .map(::std::ops::Deref::deref)
            }
        }

        impl<'a> crate::util::convert::TryAsMut<$t> for $enum {
            fn try_as_mut(&mut self) -> Option<&mut $t> {
                crate::util::convert::TryAsMut::<::std::borrow::Cow<'a, $r>>
                    ::try_as_mut(self)
                    .map(::std::borrow::Cow::to_mut)
            }
        }
    }
}