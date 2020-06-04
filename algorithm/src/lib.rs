pub struct Lines {
    list: Vec<i32>
}

impl Lines {
    fn bubble_sort_for(&mut self) {

        let new_list = &mut self.list;

        for _ in 0..new_list.len() {
            for j in 0..(&new_list.len()-1) {
                if new_list[j] > new_list[j + 1] {
                    new_list.swap(j, j+1)
                }
            }
        }
    }

    fn bubble_sort_while(&mut self) {
        let mut i = 0;
        while i < self.list.len() {
            let mut j = self.list.len() - 1;
            while j > i {
                if self.list[j-1] > self.list[j] {
                    self.list.swap(j-1, j);
                }
                j -= 1;
            }

            i += 1;
        }
    }

    fn quick_sort(&mut self, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut l = left;
        let mut r = right;
        while l < r {
            while l < r && self.list[r] >= self.list[left] {
                r -= 1;
            }
            while l < r && self.list[l] <= self.list[left] {
                l += 1;
            }
            self.list.swap(l, r);
        }
        self.list.swap(left, l);
        if l > 1 {
            self.quick_sort(left, l - 1);
        }

        self.quick_sort(r + 1, right);
    }

    fn quick_sort_new(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let pivot = self.partition(start, end);
        println!("=={}", pivot);

        self.quick_sort(start, (pivot - 1) as usize);
        self.quick_sort((pivot + 1) as usize, end);
    }

    fn partition(&mut self, start: usize, end: usize) -> i32 {
        let mut pivot = self.list[end];

        let mut index = start;

        let mut i = start;
        while i < end {
            if self.list[i] < pivot {
                self.swap(i, index);
                index += 1;
            }

            i+=1;
        }
        self.swap(index, end);
        return index as i32;
    }

    fn swap(&mut self, i: usize, j: usize ) {
        let temp = self.list[i];
        self.list[i] = self.list[j];
        self.list[j] = temp;
    }

    fn binary_search_internal(&self, num: i32, low :usize, high: usize) -> usize {
        if low > high {
           return  0;
        }

        let mid = ((low + high)/2) as usize;

        if num > self.list[mid] {
          return  self.binary_search_internal(num, mid+1, high);
        }
        if num < self.list[mid] {
          return   self.binary_search_internal(num, low, mid-1);
        }

        mid + 1

    }

}

fn bubble_for(lines: &mut Lines) {
    lines.bubble_sort_for()
}

fn bubble_while(lines: &mut Lines) {
    lines.bubble_sort_while()
}

fn quick_sort(lines: &mut Lines) {
    let last_index = lines.list.len() - 1;
    lines.quick_sort(0, last_index);
}

fn quick_other(lines: &mut Lines) {
    let last_index = lines.list.len() - 1;
    lines.quick_sort_new(0, last_index);
}

fn binary_search(lines: Lines, num: i32) ->usize {
    let len = lines.list.len();
    let key = lines.binary_search_internal(num, 0, len-1);
    key
}

mod test {
    use super::*;

    #[test]
    fn test_bubble_for() {
        let test_line = &mut Lines {
            list: vec![22, 11, 9, 10, 13, 3, 5, 1, 20],
        };

        bubble_for(test_line);

        assert_eq!(
            vec![1, 3, 5, 9, 10, 11, 13, 20, 22],
            test_line.list
        );
    }

    #[test]
    fn test_bubble_while() {
        let test_line = &mut Lines {
            list: vec![99, 22, 11, 9, 10, 13, 3, 5, 1, 20, 11, 12],
        };

        bubble_while(test_line);

        assert_eq!(
            vec![1, 3, 5, 9, 10, 11, 11, 12, 13, 20, 22, 99],
            test_line.list
        );
    }


    #[test]
    fn test_quick() {
        let test_line = &mut Lines {
            list: vec![99, 22, 11, 9, 10, 13, 3, 5, 1, 20, 11, 12],
        };

        quick_sort(test_line);

        assert_eq!(
            vec![1, 3, 5, 9, 10, 11, 11, 12, 13, 20, 22, 99],
            test_line.list
        );
    }

    #[test]
    fn test_quick_new() {
        let test_line = &mut Lines {
            list: vec![99, 22, 11, 9, 10, 13, 3, 5, 1, 20, 11, 12],
        };

        quick_other(test_line);

        assert_eq!(
            vec![1, 3, 5, 9, 10, 11, 11, 12, 13, 20, 22, 99],
            test_line.list
        );
    }

    #[test]
    fn test_binary_search() {
        let test_line = Lines {
            list: vec![1, 3, 5, 9, 10, 11, 11, 12, 13, 20, 22, 99],
        };

        let key = binary_search(test_line, 99);

        assert_eq!(
            key,
            12
        );
    }
}
