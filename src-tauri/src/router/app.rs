// router/app.rs

use super::{RouterBuilder};

pub(crate) fn mount() -> RouterBuilder {
	// getAppNameをエンドポイントとし、文字列で"rspc Test Project"を返す
	<RouterBuilder>::new().query("getAppName", |t| t(|_: (), _: ()| "rspc Test Project"))
}