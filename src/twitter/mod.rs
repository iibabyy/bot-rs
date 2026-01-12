mod scraper;
pub use scraper::TwitterScraper;

mod config {
	pub const X_LINK: &str = "https://www.twitter-viewer.com/api/x/tweet?tweetId=";
}
