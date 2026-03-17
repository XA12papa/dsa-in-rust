

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rows = matrix.len();
    let mut cols = matrix[0].len();

    let mut result  = Vec::new();


    let mut top= 0 ;
    let mut bottom = rows -1 ;

    let mut left = 0 ;
    let mut right = cols -1 ;


    while (left<= right &&top<=bottom) {
        for i in left..=right {
            result.push(matrix[top][i]);
        };

        top += 1 ;

        for i  in top..=bottom{
            result.push(matrix[i][right]);
        }  ;

        if right == 0 { break; }
        right -= 1;

        // bottom row
        if top <= bottom {
            for i in (left..=right).rev() {
                result.push(matrix[bottom][i]);
            }
            if bottom == 0 { break; }
            bottom -= 1;
        }

        // left column
        if left <= right {
            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }
    return result;
}




pub fn matrixOperations(){ 
    let s   =vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12]
    ];

}