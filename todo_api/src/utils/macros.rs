// from: https://github.com/surrealdb/surrealdb.wasm/blob/main/src/mac/mod.rs

macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
		let mut m = ::std::collections::BTreeMap::new();
        $(m.insert($k, $v);)+
        m
    }};
  }

pub(crate) use map;
