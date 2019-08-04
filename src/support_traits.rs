#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "default"), allow(dead_code))]
pub enum Trait {
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Clone,
    Copy,
    Deref,
    DerefMut,
}

impl Trait {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Trait::Debug => "Debug",
            Trait::PartialEq => "PartialEq",
            Trait::Eq => "Eq",
            Trait::PartialOrd => "PartialOrd",
            Trait::Ord => "Ord",
            Trait::Hash => "Hash",
            Trait::Default => "Default",
            Trait::Clone => "Clone",
            Trait::Copy => "Copy",
            Trait::Deref => "Deref",
            Trait::DerefMut => "DerefMut",
        }
    }

    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Trait {
        let s = s.as_ref();

        match s {
            #[cfg(feature = "Debug")]
            "Debug" => Trait::Debug,
            #[cfg(feature = "PartialEq")]
            "PartialEq" => Trait::PartialEq,
            #[cfg(feature = "Eq")]
            "Eq" => Trait::Eq,
            #[cfg(feature = "PartialOrd")]
            "PartialOrd" => Trait::PartialOrd,
            #[cfg(feature = "Ord")]
            "Ord" => Trait::Ord,
            #[cfg(feature = "Hash")]
            "Hash" => Trait::Hash,
            #[cfg(feature = "Default")]
            "Default" => Trait::Default,
            #[cfg(feature = "Clone")]
            "Clone" => Trait::Clone,
            #[cfg(feature = "Copy")]
            "Copy" => Trait::Copy,
            #[cfg(feature = "Deref")]
            "Deref" => Trait::Deref,
            #[cfg(feature = "DerefMut")]
            "DerefMut" => Trait::DerefMut,
            _ => panic!("Unsupported trait `{}`. Available traits are {:?}", s, Trait::support_traits())
        }
    }

    #[inline]
    pub fn support_traits() -> Vec<&'static str> {
        let mut traits = Vec::new();

        #[cfg(feature = "Debug")]
            {
                traits.push(Trait::Debug.as_str());
            }

        #[cfg(feature = "PartialEq")]
            {
                traits.push(Trait::PartialEq.as_str());
            }

        #[cfg(feature = "Eq")]
            {
                traits.push(Trait::Eq.as_str());
            }

        #[cfg(feature = "PartialOrd")]
            {
                traits.push(Trait::PartialOrd.as_str());
            }

        #[cfg(feature = "Ord")]
            {
                traits.push(Trait::Ord.as_str());
            }

        #[cfg(feature = "Hash")]
            {
                traits.push(Trait::Hash.as_str());
            }

        #[cfg(feature = "Default")]
            {
                traits.push(Trait::Default.as_str());
            }

        #[cfg(feature = "Clone")]
            {
                traits.push(Trait::Clone.as_str());
            }

        #[cfg(feature = "Copy")]
            {
                traits.push(Trait::Copy.as_str());
            }

        #[cfg(feature = "Deref")]
            {
                traits.push(Trait::Deref.as_str());
            }

        #[cfg(feature = "DerefMut")]
            {
                traits.push(Trait::DerefMut.as_str());
            }

        traits
    }
}

