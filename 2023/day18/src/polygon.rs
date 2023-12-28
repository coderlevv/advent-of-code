


pub fn shoelace(vertices: &[(i64, i64)]) -> i64 {
    let mut left: i64 = 0;
    let mut right: i64 = 0;
    let mut prev: (i64, i64) = vertices[0];
    for next in vertices.iter().skip(1) {
        let (x1, y1) = prev;
        let (x2, y2) = *next;
        left += x1 * y2;
        right += x2 * y1;
        prev = *next;
    }
    ((left - right) / 2).abs()
}