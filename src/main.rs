use std::fs::File;
use std::io::{BufWriter, Write};

fn makefile(name:&str, version:&str, exec:&str, icon:&str, nterm:&str, ftype:&str, fpath:&str) -> std::io::Result<()> {
    let file = File::create(fpath.trim())?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"[Desktop Entry]\n")?;
    writer.write_all(b"Encoding=UTF-8\n")?;
    writer.write_all(b"Name=")?;
    writer.write_all(name.as_bytes())?;
    writer.write_all(b"Version=")?;
    writer.write_all(version.as_bytes())?;
    writer.write_all(b"Exec=")?;
    writer.write_all(exec.as_bytes())?;
    writer.write_all(b"Icon=")?;
    writer.write_all(icon.as_bytes())?;
    writer.write_all(b"Terminal=")?;
    writer.write_all(nterm.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(b"Type=")?;
    writer.write_all(ftype.as_bytes())?;
    Ok(())
}

fn main() {
    let mut app = gfxsrc::Screen::new(30, 16, ' '.to_string());
    app.set_title("DF-FC", "#FFFFFF");
    app.addstring(0, 2, "Name:", "#FFFFFF");
    let name = app.addinput(0, 3, "==> ", "#FFFFFF");
    app.addstring(0, 4, "Version:", "#FFFFFF");
    let version = app.addinput(0, 5, "==> ", "#FFFFFF");
    app.addstring(0, 6, "Executable:", "#FFFFFF");
    let exec = app.addinput(0, 7, "==> ", "#FFFFFF");
    app.addstring(0, 8, "Icon:", "#FFFFFF");
    let icon = app.addinput(0, 9, "==> ", "#FFFFFF");
    let mut unvalidans = true;
    let mut nterm = String::new();
    while unvalidans {
        app.addstring(0, 10, "Needs Terminal [y/n]:", "#FFFFFF");
        nterm = app.addinput(0, 11, "==> ", "#FFFFFF");
        if nterm.trim()==String::from("y") || nterm.trim()==String::from("n") {
            if nterm.trim()==String::from("y"){
                nterm = String::from("true");
            }
            else {
                nterm = String::from("false");
            }
            unvalidans = false;
        } else {
            app.addstring(0, 12, "Not Valid", "#FFFFFF");
        }
    }
    app.addstring(0, 12, "Type:        ", "#FFFFFF");
    let ftype = app.addinput(0, 13, "==> ", "#FFFFFF");
    app.addstring(0, 14, "Save to:", "#FFFFFF");
    let fpath = app.addinput(0, 15, "==> ", "#FFFFFF");
    let _ = makefile(name.as_str(), &version, &exec, &&icon, &nterm, &ftype, &fpath);
}
