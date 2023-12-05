use std::{
    cmp::{max_by, min_by, Ordering},
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    hash::Hash,
    iter::{Product, Sum},
};

use anyhow::{format_err, Error, Ok as ok, Result};

use common::hash::prelude::*;

/// An interface for dealing with iterators of results and shortcuts to collect into usual types.
pub trait OkIterator<T, E>
where
    Self: Sized + Iterator<Item = Result<T, E>>,
    E: Into<Error>,
{
    #[inline]
    fn ok_count(&mut self) -> Result<usize> {
        self.ok_fold(0, |count, _value| count + 1)
    }

    #[inline]
    fn ok_last(&mut self) -> Result<Option<T>> {
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
    fn ok_collect<B>(&mut self) -> Result<B>
    where
        B: FromIterator<T>,
    {
        self.collect::<Result<_, _>>().map_err(Into::into)
    }

    // Let's make some shortcuts to usual types...
    #[inline]
    fn ok_collect_vec(&mut self) -> Result<Vec<T>> {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_array<const N: usize>(&mut self) -> Result<[T; N]> {
        self.ok_collect_vec()?
            .try_into()
            .map_err(|err: Vec<_>| format_err!("Not {} long but {}", N, err.len()))
    }

    #[inline]
    fn ok_collect_str(&mut self) -> Result<String>
    where
        String: FromIterator<T>,
    {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_vecd(&mut self) -> Result<VecDeque<T>> {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_list(&mut self) -> Result<LinkedList<T>> {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_heap(&mut self) -> Result<BinaryHeap<T>>
    where
        T: Ord,
    {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_hset(&mut self) -> Result<HashSet<T>>
    where
        T: Eq + Hash,
    {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_btset(&mut self) -> Result<BTreeSet<T>>
    where
        T: Eq + Hash + Ord,
    {
        self.ok_collect()
    }

    #[inline]
    fn ok_collect_hmap<K, V>(&mut self) -> Result<HashMap<K, V>>
    where
        T: Into<(K, V)>,
        K: Eq + Hash,
    {
        self.map(|res| res.map(Into::into)).ok_collect()
    }

    #[inline]
    fn ok_collect_btmap<K, V>(&mut self) -> Result<BTreeMap<K, V>>
    where
        T: Into<(K, V)>,
        K: Eq + Hash + Ord,
    {
        self.map(|res| res.map(Into::into)).ok_collect()
    }

    fn ok_partition<B, F>(&mut self, f: F) -> Result<(B, B)>
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
    fn ok_fold<B, F>(&mut self, init: B, mut f: F) -> Result<B>
    where
        F: FnMut(B, T) -> B,
    {
        self.try_fold(init, |accum, res| res.map(|v| f(accum, v)))
            .map_err(Into::into)
    }

    #[inline]
    fn ok_reduce<F>(&mut self, f: F) -> Result<Option<T>>
    where
        F: FnMut(T, T) -> T,
    {
        let Some(first) = self.next().transpose().map_err(Into::into)? else {
            return Ok(None);
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
    fn ok_max(&mut self) -> Result<Option<T>>
    where
        T: Ord,
    {
        self.ok_max_by(Ord::cmp)
    }

    #[inline]
    fn ok_max_by_key<B, F>(&mut self, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        Ok(self
            .map(|res| {
                let value = res.map_err(Into::into)?;
                ok((f(&value), value))
            })
            .ok_max_by(|(key1, _value1), (key2, _value2)| key1.cmp(key2))?
            .map(|(_key, value)| value))
    }

    #[inline]
    fn ok_max_by<F>(&mut self, mut compare: F) -> Result<Option<T>>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.ok_reduce(|x, y| max_by(x, y, &mut compare))
    }

    #[inline]
    fn ok_min(&mut self) -> Result<Option<T>>
    where
        T: Ord,
    {
        self.ok_min_by(Ord::cmp)
    }

    #[inline]
    fn ok_min_by_key<B, F>(&mut self, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        Ok(self
            .map(|res| {
                let value = res.map_err(Into::into)?;
                ok((f(&value), value))
            })
            .ok_min_by(|(key1, _value1), (key2, _value2)| key1.cmp(key2))?
            .map(|(_key, value)| value))
    }

    #[inline]
    fn ok_min_by<F>(&mut self, mut compare: F) -> Result<Option<T>>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.ok_reduce(|x, y| min_by(x, y, &mut compare))
    }

    // ok_unzip

    #[inline]
    fn ok_sum<S>(&mut self) -> Result<S>
    where
        S: Sum<T>,
    {
        self.sum::<Result<_, _>>().map_err(Into::into)
    }

    #[inline]
    fn ok_product<P>(&mut self) -> Result<P>
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
