use ndarray::{Array2, ArrayD, IntoDimension, Ix2, IxDyn, ScalarOperand, Slice, Zip};
use std::fmt::Debug;
use std::ops::Mul;
fn pad<T>(x: &Array2<T>, pad_width: usize, fill: T) -> Array2<T>
where
    T: Clone + Debug,
{
    let new_shape: IxDyn = x
        .shape()
        .iter()
        .map(|size| 2 * pad_width + size)
        .collect::<Vec<usize>>()
        .into_dimension();
    let mut new_array = ArrayD::<T>::from_elem(new_shape, fill)
        .into_dimensionality::<Ix2>()
        .expect("Couldn't convert back to 2D.");
    x.assign_to(
        new_array.slice_each_axis_mut(|ax| Slice::from(pad_width..pad_width + x.len_of(ax.axis))),
    );
    new_array
}
pub fn convolv<T>(map_array: &Array2<T>, kernel: &Array2<T>) -> Array2<T>
where
    T: Mul<Output = T> + Clone + Debug + Default + ScalarOperand + num_traits::Zero,
{
    let kernel_size = kernel.shape();
    let pad_width = (kernel_size[0] - 1) / 2;
    assert!(
        kernel_size.iter().all(|x| *x == kernel_size[0]),
        "Not implemented for non-square kernels"
    );
    let padded = pad(&map_array, pad_width, T::default());
    log::trace!("Padded\n\n{:?}\n\n", padded);
    let output =
        Zip::from(padded.windows(kernel.raw_dim())).map_collect(|window| (&window * kernel).sum());
    log::debug!("Convolution\n\n{:?}\n\n", output);
    output
}
pub fn erode<T>(map: &mut Array2<T>, mask: &Array2<bool>)
where
    T: Default,
{
    Zip::from(map).and(mask).for_each(|elem, b| {
        if *b {
            *elem = T::default()
        }
    });
}
