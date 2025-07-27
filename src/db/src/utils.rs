// TODO: Move this to a separate crate (it might be needed in the `api` crate).
pub fn spawn_cpu_blocking<F, T>(f: F) -> tokio::sync::oneshot::Receiver<T>
where
	F: FnOnce() -> T + Send + 'static,
	T: Send + core::fmt::Debug + 'static,
{
	let (send, recv) = tokio::sync::oneshot::channel();

	rayon::spawn(move || {
		send.send(f()).unwrap();
	});

	recv
}
