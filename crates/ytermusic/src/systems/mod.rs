use download_manager::{Downloader, DownloadManager};
use once_cell::sync::Lazy;

use crate::{
    config::DownloaderConfig,
    consts::{CACHE_DIR, CONFIG},
    DATABASE,
};

pub mod logger;
pub mod player;

pub static DOWNLOAD_MANAGER: Lazy<DownloadManager> = Lazy::new(|| {
    let downloader = match CONFIG.global.downloader {
        DownloaderConfig::Ytdlp => Downloader::YtDlp,
        #[cfg(feature = "rusty-ytdl-backend")]
        DownloaderConfig::RustyYtdl => Downloader::RustyYtdl,
        #[cfg(not(feature = "rusty-ytdl-backend"))]
        DownloaderConfig::RustyYtdl => {
            log::warn!("rusty-ytdl-backend feature not compiled in, falling back to yt-dlp");
            Downloader::YtDlp
        }
    };
    DownloadManager::new(
        CACHE_DIR.to_path_buf(),
        &DATABASE,
        CONFIG.global.parallel_downloads,
        downloader,
    )
});
