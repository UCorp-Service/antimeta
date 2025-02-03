use std::process::Command as ProcessCommand; // std::process::Command'ı yeniden adlandırıyoruz
use std::path::Path;
use clap::{Command, Arg};

// ==========================
//         ANTI META
// ==========================
//  Developer: 	Umbrella Corp
//  Version: 1.0
// ==========================

const TOOL_LOGO: &str = r#"
 @@@@@@  @@@  @@@ @@@@@@@ @@@           @@@@@@@@@@  @@@@@@@@ @@@@@@@ @@@@@@ 
@@!  @@@ @@!@!@@@   @!!   @@!           @@! @@! @@! @@!        @!!  @@!  @@@
@!@!@!@! @!@@!!@!   @!!   !!@           @!! !!@ @!@ @!!!:!     @!!  @!@!@!@!
!!:  !!! !!:  !!!   !!:   !!:           !!:     !!: !!:        !!:  !!:  !!!
 :   : : ::    :     :    :      ....... :      :   : :: ::     :    :   : :
                                 : :: : :                                   
"#;

const DEVELOPER_LOGO: &str = r#"
______               _____  __________________ ________ ________ 
___  /_ _____  __    __  / / /__  ____/__  __ \___  __ \___  __ \
__  __ \__  / / /    _  / / / _  /     _  / / /__  /_/ /__  /_/ /
_  /_/ /_  /_/ /     / /_/ /  / /___   / /_/ / _  _, _/ _  ____/ 
/_.___/ _\__, /      \____/   \____/   \____/  /_/ |_|  /_/      
        /____/                                                  
"#;

fn clean_photo_metadata(file_path: &str) {
    let output = ProcessCommand::new("exiftool")
        .arg("-all=")
        .arg(file_path)
        .output()
        .expect("ExifTool çalıştırılamadı");

    if output.status.success() {
        println!("[+] Fotoğraf metadatası temizlendi: {}", file_path);
    } else {
        eprintln!("[-] Hata oluştu: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn clean_video_metadata(file_path: &str) {
    let output = ProcessCommand::new("ffmpeg")
        .args(&["-i", file_path, "-map_metadata", "-1", "-c", "copy", "temp_output.mp4"])
        .output()
        .expect("FFmpeg çalıştırılamadı");

    if output.status.success() {
        std::fs::rename("temp_output.mp4", file_path).expect("Dosya yeniden adlandırılamadı");
        println!("[+] Video metadatası temizlendi: {}", file_path);
    } else {
        eprintln!("[-] Hata oluştu: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn main() {
    println!("{}", TOOL_LOGO);
    println!("{}", DEVELOPER_LOGO);

    let matches = Command::new("ANTI META")
        .version("1.0")
        .author("GitHub User")
        .about("Fotoğraf ve videolardan metadata temizler")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_parser(clap::value_parser!(String))
            .help("Temizlenecek dosya yolu"))
        .get_matches();

    if let Some(file) = matches.get_one::<String>("file") {
        let path = Path::new(file);
        if path.exists() {
            if let Some(ext) = path.extension() {
                match ext.to_str().unwrap().to_lowercase().as_str() {
                    "jpg" | "jpeg" | "png" | "webp" => clean_photo_metadata(file),
                    "mp4" | "mov" | "mkv" => clean_video_metadata(file),
                    _ => eprintln!("[-] Desteklenmeyen dosya formatı: {}", file),
                }
            }
        } else {
            eprintln!("[-] Dosya bulunamadı: {}", file);
        }
    } else {
        eprintln!("[-] Lütfen bir dosya belirtin --file ile.");
    }
}

