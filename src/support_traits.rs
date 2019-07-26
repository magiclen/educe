#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trait {
    Debug
}

impl Trait {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Trait::Debug => "Debug"
        }
    }

    #[inline]
    pub fn from_str<S: AsRef<str>>(s: S) -> Trait {
        let s = s.as_ref();

        match s {
            "Debug" => Trait::Debug,
            _ => panic!("Unsupported trait `{}`. Available traits are {:?}", s, Trait::support_traits())
        }
    }

    #[inline]
    pub fn support_traits() -> [&'static str; 1] {
        [Trait::Debug.as_str()]
    }
}

