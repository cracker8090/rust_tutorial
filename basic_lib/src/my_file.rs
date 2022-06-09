use std::{fs::{File, self, OpenOptions}, io::{Read, Seek, SeekFrom, Write, self, ErrorKind}};

#[test]
fn new_test() -> std::io::Result<()>{
    let _f = File::create("test666.txt")?;
    fs::remove_file("test666.txt")?;
    // fs::create_dir("tmp1/abc")?;
    fs::create_dir_all("tmp1/aaa/bbb/ccc")?;
    // fs::remove_dir("tmp1")?;
    // fs::remove_dir("tmp")?;
    // fs::remove_dir_all("tmp")?;
    // fs::remove_file("test.txt")?;

    Ok(())
}

#[test]
fn read_write_test() -> std::io::Result<()>{
    // only read, error when file no exist
    // let _file = File::open("test.txt")?;

    // creat new file,open when file exist
    // let mut _f = OpenOptions::new().read(true).write(true).create(true).open("test.txt")?;

    // append
    // let _file = OpenOptions::new().append(true).open("test.txt")?;

    // truncate clear file
    // let _file = OpenOptions::new().write(true).truncate(true).open("test.txt")?;

    // let mut f = File::open("test.txt")?;
    let mut f = OpenOptions::new().read(true).write(true).create(true).open("test.txt")?;
    // let mut buffer = [0;10];
    // let n = f.read(&mut buffer[..])?;
    // println!("The bytes: {:?}",&buffer[..n]);

    // f.seek(SeekFrom::Start(10))?;
    // f.seek(SeekFrom::End(-10))?;
    // f.seek(SeekFrom::Current(-10))?;
    let mut buf = String::new();
    // error when no utf-8
    f.read_to_string(&mut buf)?;
    println!("The buffer string: {:?}",buf);

    

    f.write(b"00write bytes")?;
    // f.write_all(buf)
    f.flush()?;
    Ok(())

}

#[test]
fn test_file() -> io::Result<()> {
    // let mut f = File::open("../file.txt").unwrap();
    let f = File::open("file1.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("file1.txt") {
                Ok(fc) => fc,
                    // println!("my test create file1.txt");
                    // fc
                // },
                Err(e) => panic!("my test create file error,{:?}",e),
            }
        },
        Err(error) => panic!("my test another file error,{:?}", error),
    };
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}