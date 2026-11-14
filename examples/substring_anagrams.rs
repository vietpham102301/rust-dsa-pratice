use dsa::algorithms::substring_anagrams::find_anagrams;

fn main() {
    let p = "abc";
    let s = "abccbabac";

    let res = find_anagrams(&s, &p);
    println!("{:?}", res)
}
