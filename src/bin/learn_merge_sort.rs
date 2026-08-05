//归并排序
fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
  let len = arr.len();
  if len > 1 {
      let mid = len / 2;
      let mut left = arr[..mid].to_vec();
      let mut right = arr[mid..].to_vec();

      merge_sort(&mut left);
      merge_sort(&mut right);

      let mut i = 0;
      let mut j = 0;
      let mut k = 0;

      while i < left.len() && j < right.len() {
          if left[i] < right[j] {
              arr[k] = left[i];
              i += 1;
          } else {
              arr[k] = right[j];
              j += 1;
          }
          k += 1;
      }

      while i < left.len() {
          arr[k] = left[i];
          i += 1;
          k += 1;
      }

      while j < right.len() {
          arr[k] = right[j];
          j += 1;
          k += 1;
      }
      println!("run once");
  }
}

fn main() {
  let mut arr = [100,99,98,97,96,95,94,93,92,91,99,90,89,87,86,85,84,83,82,81,80,79,78,77,76,75,74,73,72,71,70,5, 4, 3, 2, 1];
  merge_sort(&mut arr);
  println!("{:?}", arr);
}