use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum QueryFilter<T> {
    Val(T),
    None,
    Ignore,
}

impl<T> Default for QueryFilter<T> {
    fn default() -> Self {
        Self::Ignore
    }
}

impl<T> QueryFilter<T> {
    pub fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<QueryFilter<U>, E> {
        match self {
            Self::Val(value) => Ok(QueryFilter::Val(f(value)?)),
            Self::None => Ok(QueryFilter::None),
            Self::Ignore => Ok(QueryFilter::Ignore),
        }
    }
}
