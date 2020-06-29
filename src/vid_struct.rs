#[derive(Debug)]
pub struct VidStruct {
  pub a_q: Option<String>,
  pub a_rate: Option<String>,

  pub v_q: Option<String>,
  pub v_fps: Option<u64>,

  pub bitrate: u64,
  pub mime: String,
  pub url: String,
}