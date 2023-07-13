
mod parallel_same_folder_file;

use crate::parallel_same_folder_file::parallel_file_same_folder_function;

mod parallel_sub_folder_in_src;

use parallel_file_inside_sub_folder_function;




fn main() {
    println!("main(main.rs) Hello, world!");
    function();

    // call from parallel file same folder 
    parallel_file_same_folder_function();

    // call from sub folder
    
}

fn function(){

    println!("main(main.rs).function():: function()");
}

