use std::fs;
use std::env;
use std::path::PathBuf;
use std::io::{ Error, ErrorKind };

use dirs::data_local_dir;
use tequel::encrypt::{ TequelEncrypt, TequelEncryption };

use crate::models::User;

pub struct Fiman {

    pub data_path: PathBuf,
    pub user_data_path: PathBuf,

    pub teq: TequelEncrypt,
    pub user_private_key: String

}


impl Fiman {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let mut data_path = data_local_dir().ok_or_else(|| {
            Error::new(ErrorKind::Other, "Could not find the system data directory")
        })?;

        data_path.push("olam");

        let user_data_path = data_path.join("userdata.tql");

        let mut fiman = Self {
            data_path,
            user_data_path,

            teq: TequelEncrypt::new(),
            user_private_key: "".to_string()
        };

        fiman.user_private_key = fiman.get_machine_seed();

        Ok(fiman)

    }



    pub fn helper_file_encrypt(&mut self, content: &[u8], path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

        let encrypted = self.teq.encrypt(content, &self.user_private_key)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let json_data = serde_json::to_string(&encrypted).map_err(|e| Error::new(ErrorKind::Other, e))?;

        fs::write(&path, json_data).map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(())

    }


    pub fn write(&mut self, data: &User, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

        let json_data = serde_json::to_vec(data).map_err(|e| Error::new(ErrorKind::Other, e))?;
    
        let mut tmp_path = path.clone();
        tmp_path.set_extension("tmp");

        fs::write(&tmp_path, &json_data).map_err(|e| Error::new(ErrorKind::Other, e))?;

        fs::rename(&tmp_path, path).map_err(|e| {
            let _ = std::fs::remove_file(&tmp_path);
            e
        })?;

        self.helper_file_encrypt(&json_data, path)?;

        Ok(())
    }


    pub fn read(&mut self, path: &PathBuf) -> Result<User, Box<dyn std::error::Error>> {

        let content = fs::read_to_string(&path).map_err(|e| Error::new(ErrorKind::Other, e))?;
        let enc_struct: TequelEncryption = serde_json::from_str(&content).map_err(|e| Error::new(ErrorKind::Other, e))?;

        let decrypted = self.teq.decrypt(&enc_struct, &self.user_private_key).map_err(|e| {
            Error::new(ErrorKind::Other, format!("TEQUEL FAILED: {}", e))
        })?;

        let result = serde_json::from_str::<User>(&decrypted).map_err(|e| {
            Error::new(ErrorKind::Other, format!("TEQUEL FAILED: {}", e))
        })?;

        Ok(result)

    }


    pub fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        fs::create_dir_all(&self.data_path).map_err(|e| e)?;

        if !self.user_data_path.exists() {

            let user = User { 

                current_space_hash: "".to_string(),
                current_timeline_hash: "".to_string(),
                spaces: Vec::new()

            };

            let user_json = serde_json::to_string_pretty(&user).map_err(|e| Error::new(ErrorKind::Other, e))?;

            self.helper_file_encrypt(user_json.as_bytes(), &self.user_data_path.clone())?;

        }

        Ok(())

    }



    pub fn get_machine_seed(&self) -> String {
        let user = env::var("USERNAME")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "unknown_user".to_string());

        let computer = env::var("COMPUTERNAME")
            .or_else(|_| env::var("HOSTNAME"))
            .unwrap_or_else(|_| "unknown_machine".to_string());

        format!("{}-{}", user, computer).trim().to_lowercase()
    }

}