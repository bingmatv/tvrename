const TV: &str = "小电视";
const BUF: usize = 1641;
use std::{
    fs::*,
    io::*,
    path::{Path, MAIN_SEPARATOR},
};
fn main() {
    let mut p = String::new();
    println!("Input directory path:");
    stdin().read_line(&mut p).unwrap();
    if p.ends_with('\n') {
        p.pop();
    }
    let pa = Path::new(&p);
    let mut new;
    let mut a = 0u128;
    let tvl = TV.len();
    'b: for ch in pa.read_dir().unwrap().flatten() {
        let ch = ch.path();
        let t = ch
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .collect::<Vec<&str>>();
        if t.len() == 2
            && t[0].len() > tvl
            && t[0][..tvl] == *TV
            && t[0][tvl..].parse::<u128>().is_ok()
        {
            continue;
        }
        let che = format!(".{}", ch.extension().unwrap().to_str().unwrap());
        let ch = ch.to_str().unwrap();
        'a: loop {
            new = format!("{p}{MAIN_SEPARATOR}{TV}{a}{che}");
            for f in pa.read_dir().unwrap().flatten() {
                let f = f.path();
                let f = f.to_str().unwrap();
                if f == new {
                    println!("{f} already exists");
                    if a == u128::MAX {
                        break 'b;
                    } else {
                        a += 1
                    }
                    continue 'a;
                }
                if f != ch {
                    //compare file content
                    let mut f = File::open(f).unwrap();
                    let mut cf = File::open(ch).unwrap();
                    let fl = f.metadata().unwrap().len();
                    let cfl = cf.metadata().unwrap().len();
                    if fl != cfl {
                        continue;
                    }
                    let mut b = [0u8; BUF];
                    let mut c = [0u8; BUF];
                    let mut d = true;
                    loop {
                        let r = f.read(&mut b).unwrap();
                        if r == 0 {
                            break;
                        }
                        cf.read_exact(&mut c[..r]).unwrap();
                        if b[..r] != c[..r] {
                            d = false;
                            break;
                        }
                    }
                    if d {
                        panic!("Same file content in {ch}");
                    }
                }
            }
            rename(ch, new).unwrap();
            println!("Success");
            break;
        }
    }
}
