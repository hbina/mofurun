#[derive(Debug)]
pub enum Mofu<T> {
    // NOTE: I am not even sure what this means...
    // For now it just means to default state i.e. no information.
    Normal(Vec<T>),
    Unsafe(Vec<T>),
    Sorted(Vec<T>),
}

impl<T> Default for Mofu<T> {
    fn default() -> Self {
        Mofu::Normal(Vec::default())
    }
}

impl<T> Mofu<T> {
    pub fn guarantee_safe(self) -> Self {
        match self {
            Mofu::Unsafe(x) => Mofu::Normal(x),
            _ => self,
        }
    }

    /////////////////////////////////////////////
    /////
    ///// Vec API Mirror.
    /////
    /////////////////////////////////////////////

    pub fn push(self, item: T) -> Self {
        match self {
            Mofu::Normal(mut x) => {
                x.push(item);
                Mofu::Normal(x)
            }
            Mofu::Unsafe(mut x) => {
                x.push(item);
                Mofu::Unsafe(x)
            }
            Mofu::Sorted(mut x) => {
                x.push(item);
                Mofu::Normal(x)
            }
        }
    }

    pub fn sort(self) -> Self
    where
        T: Ord,
    {
        match self {
            Mofu::Normal(mut x) => {
                x.sort();
                Mofu::Sorted(x)
            }
            Mofu::Unsafe(mut x) => {
                x.sort();
                Mofu::Sorted(x)
            }
            Mofu::Sorted(_) => self,
        }
    }
}
