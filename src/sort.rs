use ndarray::prelude::*;
use ndarray::{s, Data, DataMut};
use rand::prelude::*;
use rand::thread_rng;

/// Methods for sorting and partitioning 1-D arrays.
pub trait Sort1dExt<A, S>
where
    S: Data<Elem = A>,
{
    /// Return the element that would occupy the `i`-th position if
    /// the array were sorted in increasing order.
    ///
    /// The array is shuffled **in place** to retrieve the desired element:
    /// no copy of the array is allocated.
    /// After the shuffling, all elements with an index smaller than `i`
    /// are smaller than the desired element, while all elements with
    /// an index greater or equal than `i` are greater than or equal
    /// to the desired element.
    ///
    /// No other assumptions should be made on the ordering of the
    /// elements after this computation.
    ///
    /// Complexity ([quickselect](https://en.wikipedia.org/wiki/Quickselect)):
    /// - average case: O(`n`);
    /// - worst case: O(`n`^2);
    /// where n is the number of elements in the array.
    ///
    /// **Panics** if `i` is greater than or equal to `n`.
    fn sorted_get_mut(&mut self, i: usize) -> A
    where
        A: Ord + Clone,
        S: DataMut;

    /// Return the index of `self[partition_index]` if `self` were to be sorted
    /// in increasing order.
    ///
    /// `self` elements are rearranged in such a way that `self[partition_index]`
    /// is in the position it would be in an array sorted in increasing order.
    /// All elements smaller than `self[partition_index]` are moved to its
    /// left and all elements equal or greater than `self[partition_index]`
    /// are moved to its right.
    /// The ordering of the elements in the two partitions is undefined.
    ///
    /// `self` is shuffled **in place** to operate the desired partition:
    /// no copy of the array is allocated.
    ///
    /// The method uses Hoare's partition algorithm.
    /// Complexity: O(`n`), where `n` is the number of elements in the array.
    /// Average number of element swaps: n/6 - 1/3 (see
    /// [link](https://cs.stackexchange.com/questions/11458/quicksort-partitioning-hoare-vs-lomuto/11550))
    ///
    /// **Panics** if `partition_index` is greater than or equal to `n`.
    fn partition_mut(&mut self, pivot_index: usize) -> usize
    where
        A: Ord + Clone,
        S: DataMut;
}

impl<A, S> Sort1dExt<A, S> for ArrayBase<S, Ix1>
where
    S: Data<Elem = A>,
{
    fn sorted_get_mut(&mut self, i: usize) -> A
    where
        A: Ord + Clone,
        S: DataMut,
    {
        let n = self.len();
        if n == 1 {
            self[0].clone()
        } else {
            let mut rng = thread_rng();
            let pivot_index = rng.gen_range(0, n);
            let partition_index = self.partition_mut(pivot_index);
            if i < partition_index {
                self.slice_mut(s![..partition_index]).sorted_get_mut(i)
            } else if i == partition_index {
                self[i].clone()
            } else {
                self.slice_mut(s![partition_index + 1..])
                    .sorted_get_mut(i - (partition_index + 1))
            }
        }
    }

    fn partition_mut(&mut self, pivot_index: usize) -> usize
    where
        A: Ord + Clone,
        S: DataMut,
    {
        let pivot_value = self[pivot_index].clone();
        self.swap(pivot_index, 0);
        let n = self.len();
        let mut i = 1;
        let mut j = n - 1;
        loop {
            loop {
                if i > j {
                    break;
                }
                if self[i] >= pivot_value {
                    break;
                }
                i += 1;
            }
            while pivot_value <= self[j] {
                if j == 1 {
                    break;
                }
                j -= 1;
            }
            if i >= j {
                break;
            } else {
                self.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        self.swap(0, i - 1);
        i - 1
    }
}
