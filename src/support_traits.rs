#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trait {
    Debug,
    PartialEq,
    Eq,
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
            "Debug" => Trait::Debug,
            "PartialEq" => Trait::PartialEq,
            "Eq" => Trait::Eq,
            "Hash" => Trait::Hash,
            "Default" => Trait::Default,
            "Clone" => Trait::Clone,
            "Copy" => Trait::Copy,
            "Deref" => Trait::Deref,
            "DerefMut" => Trait::DerefMut,
            _ => panic!("Unsupported trait `{}`. Available traits are {:?}", s, Trait::support_traits())
        }
    }

    #[inline]
    pub fn support_traits() -> [&'static str; 9] {
        [Trait::Debug.as_str(), Trait::PartialEq.as_str(), Trait::Eq.as_str(), Trait::Hash.as_str(), Trait::Default.as_str(), Trait::Clone.as_str(), Trait::Copy.as_str(), Trait::Deref.as_str(), Trait::DerefMut.as_str()]
    }
}

