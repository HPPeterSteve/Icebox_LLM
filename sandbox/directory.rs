mod clone;
mod chroot;
mod unshare;
mod seccomp;
mod network;

enum Entry {
   File(String),
   Directory(Vec<string>),
}

pub fn directory_creator(input: i32) -> Result<i32> String {
    let check = fs::metadata(path)?;

    if metadata.is.dir() {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            entries.push(scan_dir(&enter.path())?);
        } 
        Ok(Entry::Directory(Entries))

    } else {

        let name = path.file.name()
        .and_then(|os_str| os_str.to_str())
        .map(|s|s.to_string());

        match name {
            Some(n) => Ok(Entry::File(n)),
            None => {
                Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Caminho não possui um nome de arquivo válido"
            ))
            }
        }
    }
}
