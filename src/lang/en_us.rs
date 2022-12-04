use super::super::error::Err;
use super::super::io::Meg;
pub fn get_errmeg(error: &Err) -> &str {
    println!("目前无计划添加英文支持\nNo plan to support English");
    crate::lang::get_errmeg(error)
}
pub fn get_buildin_meg(meg: &Meg) -> &str {
    println!("目前无计划添加英文支持\nNo plan to support English");
    super::zh_cn::get_buildin_meg(meg)
}
