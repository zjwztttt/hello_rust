fn main() {
    for arg in std::env::args()
    {
        println!("{}", arg);
    }

    // 返回值，默认返回是0
    std::process::exit(0);
}