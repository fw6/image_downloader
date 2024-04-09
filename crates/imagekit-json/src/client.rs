use crate::{
    command::Command,
    response::{Response, ResponseIntoString},
};
pub struct Client;

impl Client {
    pub async fn run_command(input_str: &str) -> String {
        const SUBCOMMANDS_TO_CLEAN: &[&str] = &["Secrets"];
        let mut cmd_value: serde_json::Value = match serde_json::from_str(input_str) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Response::error(format!("Invalid command string: {}", e)).into_string()
            }
        };

        if let Some(cmd_value_map) = cmd_value.as_object_mut() {
            cmd_value_map.retain(|_, v| !v.is_null());

            for &subcommand in SUBCOMMANDS_TO_CLEAN {
                if let Some(cmd_value_secrets) = cmd_value_map
                    .get_mut(subcommand)
                    .and_then(|v| v.as_object_mut())
                {
                    cmd_value_secrets.retain(|_, v| !v.is_null());
                }
            }
        }

        let cmd: Command = match serde_json::from_value(cmd_value) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Response::error(format!("Invalid command value: {}", e)).into_string()
            }
        };

        match cmd {
            Command::Avatar(avatar_request) => {
                todo!()
                // let avatar = avatar_request.into_avatar();
                // let img = avatar.draw();
                // let img_data = img.to_vec();
                // let img_data_base64 = base64::encode(&img_data);
                // Response::success(img_data_base64).into_string()
            }
        }
    }
}
