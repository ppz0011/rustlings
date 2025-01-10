/*
 * @Author: ppz 2398672726@qq.com
 * @Date: 2024-12-30 01:35:37
 * @LastEditors: ppz 2398672726@qq.com
 * @LastEditTime: 2025-01-07 20:19:18
 * @FilePath: \rustlings\exercises\21_macros\macros1.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
}
