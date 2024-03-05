use std::{collections::HashMap, fs::File, hash::Hash, io::Read};

use csv::Reader;
use serde::de::DeserializeOwned;

use crate::table_reader::TableReader;

type FfxivRecordReader<R> = Reader<TableReader<R>>;

enum RecordCollection<V: DeserializeOwned, K: Hash + Eq = usize, R: Read = File> {
    Filtered(HashMap<K, V>),
    WholeTable(Vec<V>),
    Lazy(FfxivRecordReader<R>),
}

impl<V, R> RecordCollection<V, usize, R>
where
    V: DeserializeOwned,
    R: Read,
{
    fn build_collection<R1: Read>(
        mut reader: FfxivRecordReader<R1>,
        size_hint: usize,
    ) -> csv::Result<Self> {
        let mut collection = Vec::with_capacity(size_hint);

        // If the last row is empty the csv reader will fail at deserializing the final record,
        // this lets us check for that
        let mut last_error = None;
        for record in reader.deserialize() {
            // Return an error only if the error isn't on the final line
            if let Some(err) = last_error {
                return Err(err);
            }

            match record {
                Ok(val) => {
                    collection.push(val);
                }
                Err(err) => last_error = Some(err),
            }
        }

        collection.shrink_to_fit();

        Ok(Self::WholeTable(collection))
    }
}

impl<V, K, R> RecordCollection<V, K, R>
where
    V: DeserializeOwned,
    K: Hash + Eq,
    R: Read,
{
    fn collect(self, size_hint: usize) -> csv::Result<RecordCollection<V, usize, R>> {
        match self {
            Self::Filtered(val) => Ok(RecordCollection::<V, usize, R>::WholeTable(
                val.into_values().collect(),
            )),
            Self::WholeTable(val) => Ok(RecordCollection::<V, usize, R>::WholeTable(val)),
            Self::Lazy(rdr) => RecordCollection::<V, usize, R>::build_collection(rdr, size_hint),
        }
    }

    fn build_filtered<R1: Read>(
        reader: FfxivRecordReader<R1>,
        filter: &dyn Fn(&V) -> Option<K>,
        size_hint: usize,
    ) -> csv::Result<Self> {
        Self::filtered_from_iter(reader.into_deserialize(), filter, size_hint)
    }

    fn filtered_from_iter<I: Iterator<Item = csv::Result<V>>>(
        iter: I,
        filter: &dyn Fn(&V) -> Option<K>,
        size_hint: usize,
    ) -> csv::Result<Self> {
        let mut collection = HashMap::with_capacity(size_hint);

        // If the last row is empty the csv reader will fail at deserializing the final record,
        // this lets us check for that
        let mut last_error = None;
        for record in iter {
            // Return an error only if the error isn't on the final line
            if let Some(err) = last_error {
                return Err(err);
            }

            match record {
                Ok(val) => {
                    if let Some(key) = filter(&val) {
                        collection.insert(key, val);
                    }
                }
                Err(err) => last_error = Some(err),
            }
        }

        collection.shrink_to_fit();

        Ok(Self::Filtered(collection))
    }

    fn filter<K1: Hash + Eq>(
        self,
        filter: &dyn Fn(&V) -> Option<K1>,
        size_hint: usize,
    ) -> csv::Result<RecordCollection<V, K1, R>> {
        match self {
            Self::WholeTable(collection) => RecordCollection::<V, K1, R>::filtered_from_iter(
                collection.into_iter().map(|v| Ok(v)),
                filter,
                size_hint,
            ),
            Self::Filtered(collection) => RecordCollection::<V, K1, R>::filtered_from_iter(
                collection.into_values().map(|v| Ok(v)),
                filter,
                size_hint,
            ),
            Self::Lazy(rdr) => RecordCollection::<V, K1, R>::build_filtered(rdr, filter, size_hint),
        }
    }

    /// Retrieves the given item from the collection, the `idx` should be used if `WholeTable`, otherwise `Key`
    /// The arguments are a crude workaround for the lack of specialization and the fact `TypeId` requires `static`,
    /// otherwise `key` would be used for both.
    ///
    /// If the collection is `Lazy`, `None` will always be returned
    fn retrieve(&self, key: &K, idx: usize) -> Option<&V> {
        match self {
            Self::WholeTable(ref collection) => collection.get(idx),
            Self::Filtered(ref collection) => collection.get(key),
            Self::Lazy(_) => None,
        }
    }
}

fn find_item<V: DeserializeOwned, R: Read>(
    mut rdr: FfxivRecordReader<R>,
    correct: &dyn Fn(&V) -> bool,
) -> csv::Result<Option<V>> {
    // If the last row is empty the csv reader will fail at deserializing the final record,
    // this lets us check for that
    let mut last_error = None;
    for record in rdr.deserialize() {
        // Return an error only if the error isn't on the final line
        if let Some(err) = last_error {
            return Err(err);
        }

        match record {
            Ok(val) => {
                if correct(&val) {
                    return Ok(Some(val));
                }
            }
            Err(err) => last_error = Some(err),
        }
    }

    Ok(None)
}

pub struct RecordKeeper {}
