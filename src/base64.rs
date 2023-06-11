pub fn u8_to_u6(src: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in src.chunks(3) {
        if i.len() == 3 {
            res.push(
                i[0] >> 2
            );
            res.push(
                ((i[0] & 0b00000011) << 4) | (i[1]>> 4)
            );
            res.push(
                ((i[1] & 0b00001111) << 2) | (i[2]>> 6)
            );
            res.push(
                i[2] & 0b00111111
            );
        } else if i.len() == 2 {
            res.push(
                i[0] >> 2
            );
            res.push(
                ((i[0] & 0b00000011) << 4) | (i[1]>> 4)
            );
            res.push(
                (i[1] & 0b00001111) << 2
            );
            res.push(64);//'='
        } else {
            res.push(i[0] >> 2);
            res.push((i[0] & 0b00000011) << 4);
            res.push(64);//'='
            res.push(64);//'='
        }
    }
    res
}

//make sure u6.len() % 4 is 0
pub fn u6_to_u8(src: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in src.chunks(4) {
        res.push(
            (i[0] << 2) | (i[1] >> 4)
        );
        res.push(
            ((i[1] & 0b00001111) << 4) | (i[2] >> 2)
        );
        res.push(
            ((i[2] & 0b00000011) << 6) | i[3]
        );
    }
    res
}