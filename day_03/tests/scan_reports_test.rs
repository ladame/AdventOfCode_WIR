use day_03_lib::scan_reports::scan_line;


#[test]
fn scan_line_correct_test(){
    let mut total: i32 = 0;
    let content = String::from("[#from()mul(8,5)when()/}+%mul(2,4)mul(5,5)}}don't(){:,$+mul(11,8)");
    scan_line(&content, &mut total);
    assert_eq!(total, 161);
}

#[test]
fn scan_line_incorrect_test(){
    let mut total: i32 = 0;
    let content = String::from("[#from()mul(8ooo,5)when()/}+%mul(2,4)mul(5,5)}}don't(){:,$+mul(11,8)");
    scan_line(&content, &mut total);
    assert_eq!(total, 121);
}