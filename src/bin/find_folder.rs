extern crate find_folder;

use find_folder::Search;

fn main(){
    println!("{:?}", Search::Parents(3).for_folder("src"));
    let assets = find_folder::Search::Parents(3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    //println!("{}", Search::Kids(3).for_folder("examples"));
}
