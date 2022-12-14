use std::{
    cmp::{max_by, min_by, Ordering},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
    iter::{Product, Sum},
};

use common::{format_err, Error, Result};

/// An interface for dealing with iterators of results and shortcuts to collect into usual types.
pub trait OkIterator<T, E>
where
    Self: Sized + Iterator<Item = Result<T, E>>,
    E: Into<Error>,
{
    #[inline]
    fn ok_count(self) -> Result<usize> {
        self.ok_fold(0, |count, _value| count + 1)
    }

    #[inline]
    fn ok_last(self) -> Result<Option<T>> {
        self.ok_fold(None, |_, x| Some(x))
    }

    // // Iterator's version is based on "advance_by" which is unstable.
    // #[inline]
    // fn ok_nth(&mut self, n: usize) -> Result<Option<T>> {
    //     for _ in 0..n {
    //         if self.next().transpose().map_err(Into::into)?.is_none() {
    //             return Ok(None);
    //         }
    //     }
    //     self.next().transpose().map_err(Into::into)
    // }

    // Like the (unstable) `try_collect` method but only for results,
    // since it's the only thing I consider here.
    #[inline]
    fn ok_collect<B>(self) -> Result<B>
    where
        B: FromIterator<T>,
    {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    // Let's make some shortcuts to usual types...
    #[inline]
    fn ok_collect_vec(self) -> Result<Vec<T>> {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_array<const N: usize>(self) -> Result<[T; N]> {
        self.ok_collect_vec()?
            .try_into()
            .map_err(|err: Vec<_>| format_err!("Not {} long but {}", N, err.len()))
    }

    #[inline]
    fn ok_collect_str(self) -> Result<String>
    where
        String: FromIterator<T>,
    {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_vecd(self) -> Result<VecDeque<T>> {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_list(self) -> Result<LinkedList<T>> {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_heap(self) -> Result<BinaryHeap<T>>
    where
        T: Ord,
    {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_hset(self) -> Result<HashSet<T>>
    where
        T: Eq + Hash,
    {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_btset(self) -> Result<BTreeSet<T>>
    where
        T: Eq + Hash + Ord,
    {
        self.map(|res| res.map_err(Into::into)).collect()
    }

    #[inline]
    fn ok_collect_hmap<K, V>(self) -> Result<HashMap<K, V>>
    where
        T: Into<(K, V)>,
        K: Eq + Hash,
    {
        self.map(|res| res.map(Into::into).map_err(Into::into))
            .collect()
    }

    #[inline]
    fn ok_collect_btmap<K, V>(self) -> Result<BTreeMap<K, V>>
    where
        T: Into<(K, V)>,
        K: Eq + Hash + Ord,
    {
        self.map(|res| res.map(Into::into).map_err(Into::into))
            .collect()
    }

    fn ok_partition<B, F>(self, f: F) -> Result<(B, B)>
    where
        B: Default + Extend<T>,
        F: FnMut(&T) -> bool,
    {
        // `extend_one` is unstable so `extend([x; 1])` instead of of `extend_one(x)`.
        #[inline]
        fn extend<'a, T, B: Extend<T>>(
            mut f: impl FnMut(&T) -> bool + 'a,
            left: &'a mut B,
            right: &'a mut B,
        ) -> impl FnMut((), T) + 'a {
            move |(), x| {
                if f(&x) {
                    left.extend([x; 1]);
                } else {
                    right.extend([x; 1]);
                }
            }
        }

        let (mut left, mut right): (B, B) = Default::default();
        self.ok_fold((), extend(f, &mut left, &mut right))?;
        Ok((left, right))
    }

    #[inline]
    fn ok_fold<B, F>(self, init: B, mut f: F) -> Result<B>
    where
        F: FnMut(B, T) -> B,
    {
        let mut accum = init;
        for res in self {
            accum = f(accum, res.map_err(Into::into)?);
        }
        Ok(accum)
    }

    #[inline]
    fn ok_reduce<F>(mut self, f: F) -> Result<Option<T>>
    where
        F: FnMut(T, T) -> T,
    {
        let first = match self.next().transpose().map_err(Into::into)? {
            None => return Ok(None),
            Some(value) => value,
        };
        self.ok_fold(first, f).map(Some)
    }

    // I can't make my own `ok_try_fold` derived from `try_fold` as it would use some unstable
    // feature but I can use `all`/`any`/... which use it, it's only a bit more verbose.
    #[inline]
    fn ok_all<F>(&mut self, mut f: F) -> Result<bool>
    where
        F: FnMut(T) -> bool,
    {
        let mut err = None;
        let res_all = self.all(|res| match res {
            Ok(value) => f(value),
            Err(e) => {
                err = Some(e.into());
                false
            }
        });
        err.map_or(Ok(res_all), Err)
    }

    #[inline]
    fn ok_any<F>(&mut self, mut f: F) -> Result<bool>
    where
        F: FnMut(T) -> bool,
    {
        let mut err = None;
        let res_any = self.any(|res| match res {
            Ok(value) => f(value),
            Err(e) => {
                err = Some(e.into());
                false
            }
        });
        err.map_or(Ok(res_any), Err)
    }

    #[inline]
    fn ok_find<P>(&mut self, mut predicate: P) -> Result<Option<T>>
    where
        P: FnMut(&T) -> bool,
    {
        let mut err = None;
        let res_pos = self.find_map(|res| match res {
            Ok(value) => predicate(&value).then_some(value),
            Err(e) => {
                err = Some(e.into());
                None
            }
        });
        err.map_or(Ok(res_pos), Err)
    }

    #[inline]
    fn ok_find_map<B, F>(&mut self, mut f: F) -> Result<Option<B>>
    where
        F: FnMut(T) -> Option<B>,
    {
        let mut err = None;
        let res_pos = self.find_map(|res| match res {
            Ok(value) => f(value),
            Err(e) => {
                err = Some(e.into());
                None
            }
        });
        err.map_or(Ok(res_pos), Err)
    }

    #[inline]
    fn ok_position<P>(&mut self, mut predicate: P) -> Result<Option<usize>>
    where
        P: FnMut(T) -> bool,
    {
        let mut err = None;
        let res_pos = self.position(|res| match res {
            Ok(value) => predicate(value),
            Err(e) => {
                err = Some(e.into());
                false
            }
        });
        err.map_or(Ok(res_pos), Err)
    }

    #[inline]
    fn ok_rposition<P>(&mut self, mut predicate: P) -> Result<Option<usize>>
    where
        Self: ExactSizeIterator + DoubleEndedIterator,
        P: FnMut(T) -> bool,
    {
        let mut err = None;
        let res_rpos = self.rposition(|res| match res {
            Ok(value) => predicate(value),
            Err(e) => {
                err = Some(e.into());
                false
            }
        });
        err.map_or(Ok(res_rpos), Err)
    }

    #[inline]
    fn ok_max(self) -> Result<Option<T>>
    where
        T: Ord,
    {
        self.ok_max_by(Ord::cmp)
    }

    #[inline]
    fn ok_max_by_key<B, F>(self, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        Ok(self
            .map(|res| {
                let value = res.map_err(Into::into)?;
                common::Ok((f(&value), value))
            })
            .ok_max_by(|(key1, _value1), (key2, _value2)| key1.cmp(key2))?
            .map(|(_key, value)| value))
    }

    #[inline]
    fn ok_max_by<F>(self, mut compare: F) -> Result<Option<T>>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.ok_reduce(|x, y| max_by(x, y, &mut compare))
    }

    #[inline]
    fn ok_min(self) -> Result<Option<T>>
    where
        T: Ord,
    {
        self.ok_min_by(Ord::cmp)
    }

    #[inline]
    fn ok_min_by_key<B, F>(self, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        Ok(self
            .map(|res| {
                let value = res.map_err(Into::into)?;
                common::Ok((f(&value), value))
            })
            .ok_min_by(|(key1, _value1), (key2, _value2)| key1.cmp(key2))?
            .map(|(_key, value)| value))
    }

    #[inline]
    fn ok_min_by<F>(self, mut compare: F) -> Result<Option<T>>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.ok_reduce(|x, y| min_by(x, y, &mut compare))
    }

    // ok_unzip

    #[inline]
    fn ok_sum<S>(self) -> Result<S>
    where
        S: Sum<T>,
    {
        self.sum::<Result<_, _>>().map_err(Into::into)
    }

    #[inline]
    fn ok_product<P>(self) -> Result<P>
    where
        P: Product<T>,
    {
        self.product::<Result<_, _>>().map_err(Into::into)
    }

    // cmp cmp_by partial_cmp partial_cmp_by eq eq_by ne lt le gt ge
}

impl<T, E, It> OkIterator<T, E> for It
where
    It: Sized + Iterator<Item = Result<T, E>>,
    E: Into<Error>,
{
}
