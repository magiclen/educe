#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trait {
    Debug,
    PartialEq,
    Eq,
    Hash,
}

impl Trait {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Trait::Debug => "Debug",
            Trait::PartialEq => "PartialEq",
            Trait::Eq => "Eq",
            Trait::Hash => "Hash",
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
            _ => panic!("Unsupported trait `{}`. Available traits are {:?}", s, Trait::support_traits())
        }
    }

    #[inline]
    pub fn support_traits() -> [&'static str; 4] {
        [Trait::Debug.as_str(), Trait::PartialEq.as_str(), Trait::Eq.as_str(), Trait::Hash.as_str()]
    }
}

