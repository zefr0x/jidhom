fn main() {
	// Export build metadata as environment variables for cargo
	shadow_rs::ShadowBuilder::builder()
		.build_pattern(shadow_rs::BuildPattern::Lazy)
		.build()
		.unwrap();
}
