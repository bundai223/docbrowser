// router/mod.rs

use std::sync::Arc;

mod app;

pub type Router = rspc::Router;
pub(crate) type RouterBuilder = rspc::RouterBuilder;

pub(crate) fn mount() -> Arc<Router> {
	let config = rspc::Config::new().set_ts_bindings_header("/* eslint-disable */"); // ①

	let config = config.export_ts_bindings(
		std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/rspc/bindings.ts"), // ②
	);

	<Router>::new()
		.config(config)
		.merge("app.", app::mount()) // ③
		.build()
		.arced()
}