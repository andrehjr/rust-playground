use std::path::Path;

fn main() {
    if let Some(root) = Path::new("/foo/bar").components().next() {
        let meh = root.as_os_str().to_str().unwrap();
        println!("{:?}", "/foo/bar".trim_start_matches(meh));
    }

    println!("->>> {:?}", Path::new("/foo").as_os_str());
    println!("->>> {:?}", Path::new("/foo/bar").as_os_str());


    println!("{:?}", Path::new("/foo").components());
    println!("{:?}", Path::new("/foo/bar").components().map(|element| element.as_os_str() ).collect::<Vec<_>>());


    println!("{:?}", Path::new("//foo").components());
    println!("{:?}", Path::new("//foo/bar").components().map(|element| element.as_os_str() ).collect::<Vec<_>>());

    println!("{:?}", Path::new("C:/foo").components());
    println!("{:?}", Path::new("C://foo/bar").components().map(|element| element.as_os_str() ).collect::<Vec<_>>());

}