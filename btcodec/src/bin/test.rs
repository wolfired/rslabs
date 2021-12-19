use btcodec::BTType;

fn main() {
    // let raw = include_bytes!("./mega.torrent");
    // let bytes = &(*raw)[..];
    let bytes = &(*b"10:aaaaaaaaaa1:b2:cd3:efg4:hijk5:lmnop")[..];

    BTType::from_raw(bytes);
}
