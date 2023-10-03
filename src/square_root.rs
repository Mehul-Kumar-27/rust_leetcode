pub(crate) struct SquareRoot;

impl SquareRoot {
    pub fn calculate(&self, x: i32, start: i64, end: i64) -> i32 {
        if start <= end {
            let mid = start + (end - start) / 2;
            let square = mid * mid;

            if square == (i64::from(x)) {
                return mid as i32;
            } else if square > (i64::from(x)) {
                return self.calculate(x, start, mid - 1);
            } else {
                return self.calculate(x, mid + 1, end);
            }
        }

        end as i32
    }

    pub fn my_sqrt(&self, x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let end = (x / 2) as i64;
        self.calculate(x, 0, end)
    }
}
