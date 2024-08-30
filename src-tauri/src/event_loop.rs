// use tauri::AppHandle;

// use crate::log_reader::LogReader;

// pub fn get_loop(app: &AppHandle) {
//     let file_path = String::from(r"C:\Users\Oery\AppData\Roaming\.oerymc\logs\latest.log");
//     let mut log_reader = LogReader::new(file_path).unwrap();

//     std::thread::spawn(move || loop {
//         let _ = log_reader.read(&app);
//         std::thread::sleep(std::time::Duration::from_millis(100));
//     });
// }
