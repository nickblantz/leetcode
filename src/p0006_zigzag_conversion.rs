struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let len = s.len();
        let rows = num_rows as usize;
        let zag = rows - 2;
        let zigzag = rows + zag;
        let cols = (len + zigzag - 1) / zigzag;
        let chars = s.chars().collect::<Vec<char>>();
        let mut result = String::with_capacity(len);

        for i in 0..cols {
            result.push(chars[i * zigzag]);
        }

        for i in 0..zag {
            let mut n = i + 1;
            let x = 2 * (zag - i);
            let y = 2 * (i + 1);

            if n < len {
                result.push(chars[n]);
            }

            for j in 0..(cols * 2) {
                n += if j % 2 == 0 { x } else { y };

                if n < len {
                    result.push(chars[n]);
                }
            }
        }

        for i in 0..cols {
            let n = i * zigzag + rows - 1;

            if n < len {
                result.push(chars[n]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            String::from("A"),
            Solution::convert("A".into(), 1)
        );
        assert_eq!(
            String::from("ABCDE"),
            Solution::convert("ABCDE".into(), 1)
        );
        assert_eq!(
            String::from("A"),
            Solution::convert("A".into(), 3)
        );
        assert_eq!(
            String::from("AB"),
            Solution::convert("AB".into(), 3)
        );
        assert_eq!(
            String::from("ABC"),
            Solution::convert("ABC".into(), 3)
        );
        assert_eq!(
            String::from("ABDC"),
            Solution::convert("ABCD".into(), 3)
        );
        assert_eq!(
            String::from("PAHNAPLSIIGYIR"),
            Solution::convert("PAYPALISHIRING".into(), 3)
        );
        assert_eq!(
            String::from("PINALSIGYAHRPI"),
            Solution::convert("PAYPALISHIRING".into(), 4)
        );
        assert_eq!(
            String::from("PHRASIURYIRCEYPLIGNLANT"),
            Solution::convert("PAYPALISHIRINGCURRENTLY".into(), 5)
        );
        assert_eq!(
            String::from("PHRASIURYIRCEPLIGNANT"),
            Solution::convert("PAYPALISHIRINGCURRENT".into(), 5)
        );
        assert_eq!(
            String::from("PHRASIUYIRCPLIGAN"),
            Solution::convert("PAYPALISHIRINGCUR".into(), 5)
        );
        assert_eq!(
            String::from("PHASIUYIRCPLIGAN"),
            Solution::convert("PAYPALISHIRINGCU".into(), 5)
        );
    }
}
